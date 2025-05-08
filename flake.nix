{
  description = "StockSync Nix flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          zsh
          just
          bun
          
          rustc
          cargo
          rustfmt
          rust-analyzer
          clippy
        ];

        shellHook = ''
          exec zsh
        '';
      };
    };
}
