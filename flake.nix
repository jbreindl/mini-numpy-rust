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

      libs = with pkgs; [
        stdenv.cc.cc.lib
        zlib
        openssl
        libffi
        glibc
        gdb
      ];
    in
    {
      devShells.${system}.default =
        with pkgs;
        mkShell {
          buildInputs = [
            python3
            python3Packages.python-dateutil
            rust-bin.stable.latest.default
            just
            maturin
            uv
            gdb
          ];
          env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
          # NOTE: the $SHELL variable wasn't playing nicely
          shellHook = ''
            exec zsh
          '';
        };
    };
}
