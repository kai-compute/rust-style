"""Compile and execute the independent Rust examples in FUNCTIONAL.md."""

import argparse
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

from verify import ROOT, check_documents, check_remote_links, read_markdown, require


def rust_examples(tokens):
    result = [token for token in tokens if token.type == "fence" and token.info == "rust"]
    require(len(result) == 25, f"Expected 25 Rust examples, found {len(result)}")
    for number, token in enumerate(result, 1):
        require(token.content.startswith(f"// Example {number:02d}:"), f"Misnumbered example {number}")
    return result


def toolchain(msrv):
    env = os.environ.copy()
    expected = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    if msrv:
        env["PATH"] = env["RUST_STYLE_MSRV_BIN"] + os.pathsep + env["PATH"]
        expected = "1.85.0"
    compiler = shutil.which("rustc", path=env["PATH"])
    require(compiler is not None, "rustc is missing; enter the devenv environment")
    version = subprocess.check_output([compiler, "--version"], env=env, text=True)
    require(version.split()[1] == expected, f"Expected Rust {expected}, found {version.strip()}")
    print(version.strip(), flush=True)
    return compiler, env


def formatted(source):
    return subprocess.check_output(
        ["rustfmt", "--config-path", str(ROOT / "rustfmt.toml"), "--emit", "stdout"],
        input=source, text=True, timeout=30,
    )


def format_examples(tokens):
    toolchain(False)
    path = ROOT / "FUNCTIONAL.md"
    lines = path.read_text().splitlines(keepends=True)
    changed = []
    for number, token in reversed(list(enumerate(rust_examples(tokens), 1))):
        updated = formatted(token.content)
        if updated != token.content:
            start, end = token.map
            lines[start + 1:end - 1] = updated.splitlines(keepends=True)
            changed.append(number)
    path.write_text("".join(lines))
    print(f"Formatted examples: {sorted(changed)}", flush=True)


def run(command, directory, env, timeout):
    result = subprocess.run(command, cwd=directory, env=env, capture_output=True, text=True, timeout=timeout)
    if result.stderr:
        print(result.stderr, end="", flush=True)
    if result.returncode:
        print(result.stdout, end="", flush=True)
        raise subprocess.CalledProcessError(result.returncode, command)
    return result.stdout


def check_rust(tokens, msrv):
    compiler, env = toolchain(msrv)
    examples = rust_examples(tokens)
    for token in tokens:
        if token.type == "fence" and token.info == "sh":
            subprocess.run(["bash", "-n"], input=token.content, text=True, check=True)
    with tempfile.TemporaryDirectory(prefix="fp-rust-") as directory:
        workspace = Path(directory)
        for number, token in enumerate(examples, 1):
            if not msrv:
                require(formatted(token.content) == token.content,
                        f"Example {number}: run python scripts/verify_functional.py --format")
            folder = workspace / f"example{number:02d}"
            folder.mkdir()
            source = folder / "main.rs"
            regression = ROOT / "validation" / "functional" / f"example{number:02d}.rs"
            suffix = ""
            if regression.is_file():
                suffix = "\n#[cfg(test)]\nmod regression;\n"
                shutil.copyfile(regression, folder / "regression.rs")
                if not msrv:
                    require(formatted(regression.read_text()) == regression.read_text(),
                            f"{regression}: not formatted")
            source.write_text(token.content + suffix)
            for profile, optimization, overflow in [("debug", "0", "yes"), ("release", "3", "no")]:
                binary = folder / profile
                flags = ["--edition=2024", "--forbid=unsafe_code", "--deny=warnings", "-C", f"opt-level={optimization}",
                         "-C", f"overflow-checks={overflow}"]
                run([compiler, *flags, str(source), "-o", str(binary)], folder, env, 120)
                run([str(binary)], folder, env, 30)
                if regression.is_file():
                    test_binary = folder / f"{profile}-tests"
                    run([compiler, *flags, "--test", str(source), "-o", str(test_binary)], folder, env, 120)
                    output = run([str(test_binary)], folder, env, 30)
                    print(next(line for line in output.splitlines() if line.startswith("test result:")), flush=True)
            print(f"Example {number:02d}: debug and release passed", flush=True)
    print("All 25 examples compiled and executed in both profiles.", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--msrv", action="store_true")
    mode.add_argument("--links", action="store_true")
    mode.add_argument("--format", action="store_true")
    args = parser.parse_args()
    _, urls = check_documents()
    tokens = read_markdown(ROOT / "FUNCTIONAL.md")
    if args.links:
        check_remote_links(urls)
    elif args.format:
        format_examples(tokens)
    else:
        check_rust(tokens, args.msrv)


if __name__ == "__main__":
    main()
