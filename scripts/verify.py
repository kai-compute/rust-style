"""Validate the Markdown sources and exercise README examples without copies."""

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import tomllib
from urllib.parse import unquote, urlsplit, urlunsplit
from urllib.request import Request, urlopen

from bs4 import BeautifulSoup
from markdown_it import MarkdownIt
from mdit_py_plugins.footnote import footnote_plugin


ROOT = Path(__file__).resolve().parents[1]
PARSER = MarkdownIt("commonmark").enable("table").use(footnote_plugin)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_markdown(path):
    text = path.read_text(encoding="utf-8")
    require(text.endswith("\n"), f"{path}: missing final newline")
    require("\r" not in path.read_bytes().decode("utf-8"), f"{path}: non-LF newline")
    for number, line in enumerate(text.splitlines(), 1):
        require(line == line.rstrip(), f"{path}:{number}: trailing whitespace")
    tokens = PARSER.parse(text)
    for token in tokens:
        for child in token.children or []:
            if child.type == "text":
                require(not re.search(r"\[\^[^\]]+\]", child.content), f"{path}: unresolved footnote in {child.content}")
    return tokens


def headings(tokens):
    anchors = set()
    for index, token in enumerate(tokens):
        if token.type != "heading_open":
            continue
        children = tokens[index + 1].children or []
        title = "".join(child.content for child in children if child.type in {"text", "code_inline"})
        base = "".join(char for char in title.lower() if char.isalnum() or char in " _-")
        base = base.replace(" ", "-")
        anchor, suffix = base, 0
        while anchor in anchors:
            suffix += 1
            anchor = f"{base}-{suffix}"
        anchors.add(anchor)
    explicit = []
    for token in tokens:
        for fragment in [token, *(token.children or [])]:
            if fragment.type in {"html_block", "html_inline"}:
                explicit.extend(node["id"] for node in BeautifulSoup(fragment.content, "html.parser").select("[id]"))
    require(len(explicit) == len(set(explicit)), "Duplicate explicit HTML anchors")
    return anchors | set(explicit)


def links(tokens):
    for token in tokens:
        if token.type == "link_open":
            yield token.attrGet("href")
        if token.children:
            yield from links(token.children)


def check_documents():
    documents = {path: read_markdown(path) for path in [ROOT / "README.md", ROOT / "FUNCTIONAL.md", *sorted((ROOT / "docs").glob("*.md"))]}
    external = set()
    count = 0
    for path, tokens in documents.items():
        for href in links(tokens):
            parsed = urlsplit(href)
            if parsed.scheme in {"http", "https"}:
                external.add(href)
                continue
            require(not parsed.scheme and not parsed.netloc, f"{path}: unsupported link {href}")
            target = (path.parent / unquote(parsed.path)).resolve() if parsed.path else path
            require(target.is_relative_to(ROOT) and target.is_file(), f"{path}: missing local link {href}")
            if parsed.fragment:
                target_tokens = documents.get(target)
                if target_tokens is None:
                    target_tokens = read_markdown(target)
                require(unquote(parsed.fragment) in headings(target_tokens), f"{path}: missing anchor {href}")
            count += 1
    print(f"Markdown: {len(documents)} documents, {count} local links, {len(external)} external URLs", flush=True)
    return documents[ROOT / "README.md"], external


def check_remote_links(urls):
    for url in sorted(urls):
        parsed = urlsplit(url)
        request = Request(urlunsplit(parsed._replace(fragment="")), headers={"User-Agent": "rust-style-link-check/1.0"})
        with urlopen(request, timeout=30) as response:
            content = response.read()
            content_type = response.headers.get_content_type()
            if content_type == "text/html":
                page = BeautifulSoup(content, "html.parser")
                if parsed.fragment:
                    fragment = unquote(parsed.fragment)
                    require(page.find(id=fragment) is not None or page.find(attrs={"name": fragment}) is not None,
                            f"Missing remote anchor: {url}")
                title = page.title.get_text(" ", strip=True) if page.title else "HTML"
            else:
                title = content_type
            print(f"{response.status} {url} [{title}]", flush=True)
    print(f"All {len(urls)} external URLs passed.", flush=True)


def run(command, *, cwd, env):
    print("+ " + " ".join(map(str, command)), flush=True)
    subprocess.run(command, cwd=cwd, env=env, check=True)


def examples(tokens):
    section = ""
    preceding = ""
    result = []
    for index, token in enumerate(tokens):
        if token.type == "heading_open":
            section = tokens[index + 1].content.split(".", 1)[0]
        elif token.type == "inline":
            preceding = token.content
        elif token.type == "fence":
            result.append((section, token, preceding))
            preceding = ""
    return result


def check_rust(tokens, msrv):
    blocks = examples(tokens)
    manifests = [token.content for section, token, _ in blocks if section == "E02" and token.info == "toml"]
    require(len(manifests) == 3, "E02 must provide workspace, member, and formatter TOML blocks")
    for _, token, _ in blocks:
        if token.info == "toml":
            tomllib.loads(token.content)
        elif token.info == "sh":
            subprocess.run(["bash", "-n"], input=token.content, text=True, check=True)

    version = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    env = os.environ.copy()
    if msrv:
        version = tomllib.loads(manifests[0])["workspace"]["package"]["rust-version"] + ".0"
        msrv_bin = env["RUST_STYLE_MSRV_BIN"]
        env["PATH"] = msrv_bin + os.pathsep + env["PATH"]
    env["RUSTC"] = shutil.which("rustc", path=env["PATH"])
    env["RUSTDOC"] = shutil.which("rustdoc", path=env["PATH"])
    env["CARGO_NET_OFFLINE"] = "true"
    actual = subprocess.check_output([env["RUSTC"], "--version"], env=env, text=True)
    require(actual.split()[1] == version, f"Expected Rust {version}, found {actual.strip()}")
    print(actual.strip(), flush=True)

    with tempfile.TemporaryDirectory(prefix="rust-style-") as directory:
        workspace = Path(directory)
        env["CARGO_TARGET_DIR"] = str(workspace / "target")
        member = workspace / "crates/frame-codec"
        source = member / "src"
        source.mkdir(parents=True)
        (workspace / "Cargo.toml").write_text(manifests[0])
        (member / "Cargo.toml").write_text(manifests[1])
        (workspace / "rustfmt.toml").write_text(manifests[2])
        shutil.copyfile(ROOT / "validation/Cargo.lock", workspace / "Cargo.lock")
        modules = []
        excerpts = 0
        for section, token, preceding in blocks:
            if token.info != "rust":
                continue
            excerpt = "excerpt" in preceding.lower()
            require(re.fullmatch(r"[DB]\d{2}", section), f"Unclassified Rust example: {section}")
            name = section.lower()
            destination = (workspace if excerpt else source) / f"{name}.rs"
            require(not destination.exists(), f"Multiple Rust examples in {section}: assign distinct module names")
            destination.write_text(token.content)
            if excerpt:
                excerpts += 1
                if not msrv:
                    run(["rustfmt", "--check", "--config-path", str(workspace / "rustfmt.toml"), str(destination)], cwd=workspace, env=env)
            else:
                modules.append(f"/// Example from {section}.\npub mod {name};\n")
        require(modules, "No self-contained Rust examples were found")
        (source / "lib.rs").write_text("//! Examples extracted from the Rust Style README.\n\n" + "\n".join(modules))
        (member / "tests").mkdir()
        shutil.copyfile(ROOT / "validation/regressions.rs", member / "tests/regressions.rs")
        print(f"Rust examples: {len(modules)} complete modules, {excerpts} format-only excerpt(s)", flush=True)
        if msrv:
            commands = [
                ["cargo", "check", "--workspace", "--all-targets", "--locked"],
                ["cargo", "test", "--workspace", "--all-targets", "--locked"],
                ["cargo", "test", "--workspace", "--doc", "--locked"],
            ]
            for command in commands:
                run(command, cwd=workspace, env=env)
        else:
            gates = [token.content for section, token, _ in blocks if section == "E01" and token.info == "sh"]
            require(len(gates) == 1, "E01 must provide exactly one shell conformance gate")
            # Execute the maintained example itself so its commands cannot drift.
            print("+ E01 conformance gate (verbatim)", flush=True)
            subprocess.run(["bash", "-eu", "-c", gates[0]], cwd=workspace, env=env, check=True)
        check_edition_contracts(workspace, env)


def check_edition_contracts(workspace, env):
    fixtures = [
        ("environment", 'pub fn change() { std::env::set_var("RUST_STYLE_TEST", "1"); }', "E0133", None),
        ("extern_block", 'extern "C" { fn example(); }', None, "extern blocks must be unsafe"),
        ("async_dyn", "pub trait Work { async fn run(&self); }\npub fn consume(_: &dyn Work) {}", "E0038", None),
    ]
    for name, code, expected_code, expected_message in fixtures:
        path = workspace / f"{name}.rs"
        path.write_text(code + "\n")
        result = subprocess.run(
            [env["RUSTC"], "--edition=2024", "--crate-type=lib", "--emit=metadata", "--error-format=json", str(path)],
            cwd=workspace, env=env, capture_output=True, text=True,
        )
        errors = [json.loads(line) for line in result.stderr.splitlines() if line.startswith("{")]
        matches = [error for error in errors if error.get("level") == "error"
                   and (not expected_code or (error.get("code") or {}).get("code") == expected_code)
                   and (not expected_message or expected_message in error.get("message", ""))]
        require(result.returncode != 0 and matches, f"{name}: did not fail for the intended reason\n{result.stderr}")
        print(f"Compile-fail: {name} rejected for the expected reason", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--msrv", action="store_true", help="check and test using the Nix-provided MSRV")
    mode.add_argument("--links", action="store_true", help="check external URLs and fragments over the network")
    args = parser.parse_args()
    tokens, urls = check_documents()
    if args.links:
        check_remote_links(urls)
    else:
        check_rust(tokens, args.msrv)
        from verify_functional import check_rust as check_functional

        check_functional(read_markdown(ROOT / "FUNCTIONAL.md"), args.msrv)


if __name__ == "__main__":
    main()
