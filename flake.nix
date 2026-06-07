{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    {
      devShells.${system}.default =
        with pkgs;
        mkShell {
          buildInputs = [
            python3
            python3Packages.python-dateutil
            rust-bin.beta.latest.default
            just
          ];

          # NOTE: the $SHELL variable wasn't playing nicely
          shellHook = ''
            exec zsh
          '';
        };
    };
}
