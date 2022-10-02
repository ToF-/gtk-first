let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/22.05.tar.gz")) {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.rust.packages.stable.rustPlatform.rustcSrc
    pkgs.rust.packages.stable.rustPlatform.rustLibSrc
    ];
  };
in pkgs.mkShell {
  buildInputs = [
    rust-toolchain
    pkgs.glib
    pkgs.pkg-config
    pkgs.gtk4
    pkgs.jetbrains.idea-ultimate
  ];

  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela.
  RUST_SRC_PATH = "${rust-toolchain}";
}
