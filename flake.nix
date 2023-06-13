{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, devenv, systems, ... }@inputs:
    let 
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in {
      devShells = forEachSystem (system:
        let 
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;

            modules = [{
              packages = [
                pkgs.openssl
              ];

              languages.rust = {
                enable = true;
                version = "latest";
              };

              pre-commit.hooks = {
                clippy.enable = true;
                rustfmt.enable = true;
              };
            }];
          };
        });
    };
}