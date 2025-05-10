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
          rustup
          python313
          uv
        ];

        # shellHook = ''
        #   exec zsh
        # '';

        # # Ensure rustup is initialized
        # if ! command -v rustc &> /dev/null; then
        #   rustup default stable
        # fi
      };
    };
}
