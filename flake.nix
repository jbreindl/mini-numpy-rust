{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.python3
          pkgs.python3Packages.dateutil

        ];
        nativeBuildInputs = with pkgs; [
          just
        ];
        # NOTE: the $SHELL variable wasn't playing nicely
        shellHook = ''
          exec zsh
        '';
      };
    };
}
