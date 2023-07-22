{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ rustc cargo gcc rust-analyzer rustfmt ];
}
