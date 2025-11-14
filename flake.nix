{
  description = "Utility for patching binary files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, ... } @ inputs: {
    packages.aarch64-darwin.default =
    let
      pkgs = nixpkgs.legacyPackages.aarch64-darwin;
    in
    pkgs.rustPlatform.buildRustPackage {
      pname = "npatch";
      version = "0.1";
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };
  };
}