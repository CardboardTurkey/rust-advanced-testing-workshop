let
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ oxalica_overlay ]; };

in
pkgs.mkShell {
  name = "testing-workshop";
  packages = with pkgs; [
    (rust-bin.nightly.latest.default.override { extensions = [ "rust-src" ]; })
    openssl
    pkg-config
    cargo-insta
  ];
}
