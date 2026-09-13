{ pkgs, inputs, ... }:

let
  python = pkgs.python3.withPackages (ps: [
    ps.markdown-it-py
    ps.mdit-py-plugins
    ps.beautifulsoup4
  ]);
  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
  msrv = rustBin.stable."1.85.0".minimal;
in
{
  packages = [
    python
    pkgs.direnv
    pkgs.git
    pkgs.nixfmt
    pkgs.shellcheck
  ];

  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  env.RUST_STYLE_MSRV_BIN = "${msrv}/bin";

  scripts.verify.exec = ''
    set -euo pipefail
    cd "$DEVENV_ROOT"
    nixfmt --check devenv.nix
    shellcheck .envrc
    python scripts/verify.py
  '';

  scripts.verify-msrv.exec = ''
    set -euo pipefail
    cd "$DEVENV_ROOT"
    python scripts/verify.py --msrv
  '';

  scripts.verify-links.exec = ''
    set -euo pipefail
    cd "$DEVENV_ROOT"
    python scripts/verify.py --links
  '';

  enterTest = ''
    verify
    verify-msrv
  '';
}
