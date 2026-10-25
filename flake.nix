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
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          inherit (pkgs) lib stdenv;

          libs =
            with pkgs;
            [
              zlib
              openssl
              libffi
              lldb
            ]
            ++ lib.optionals stdenv.isLinux [
              stdenv.cc.cc.lib
              glibc
              gdb
            ];
        in
        {
          default = pkgs.mkShell {
            buildInputs =
              with pkgs;
              [
                python3
                python3Packages.python-dateutil
                rust-bin.stable.latest.default
                just
                maturin
                uv
                lldb
              ]
              ++ lib.optionals stdenv.isLinux [ gdb ];

            # LD_LIBRARY_PATH is Linux-only; macOS uses DYLD_*, which SIP
            # strips for system binaries, so it's skipped there.
            env = lib.optionalAttrs stdenv.isLinux {
              LD_LIBRARY_PATH = lib.makeLibraryPath libs;
            };

            # NOTE: the $SHELL variable wasn't playing nicely
            shellHook = ''
              exec zsh
            '';
          };
        }
      );
    };
}
