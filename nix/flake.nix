{
  description = "chromamancer — dev shell for hacking; standalone CLI is the bootstrap delivery (no HM/NixOS module yet)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "aarch64-linux"
        "x86_64-linux"
      ];
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
            ];
          };
        }
      );

      # Future: expose generated theme artifacts as packages.
      # packages = forAllSystems (system: { ... });

      # Future: Home Manager integration.
      # homeManagerModules.default = import ./modules/home-manager.nix;

      # Future: NixOS module for system-level hooks.
      # nixosModules.default = import ./modules/nixos.nix;
    };
}
