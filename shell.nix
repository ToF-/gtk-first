let
  # Last updated: 2/26/21. Update as necessary from https://status.nixos.org/...
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/22.05.tar.gz")) {};

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.glib
    pkgs.pkg-config
    pkgs.gtk3
    pkgs.jetbrains.idea-ultimate
  ];

  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
