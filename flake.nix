{
  description = "devshell for flxor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11"; 
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];

      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      pkgs = forAllSystems (system:
        import nixpkgs {
          inherit system;
        }
      );

    in {
      devShells = forAllSystems (system:
        let
          currentPkgs = pkgs.${system};
          devShellArgs = {
            atkmm = currentPkgs.atkmm;
            cairo = currentPkgs.cairo;
            gcc = currentPkgs.gcc;
            gdk-pixbuf = currentPkgs.gdk-pixbuf;
            glib = currentPkgs.glib;
            gtk3 = currentPkgs.gtk3;
            mkShell = currentPkgs.mkShell;
            openssl = currentPkgs.openssl;
            pango = currentPkgs.pango;
            pkg-config = currentPkgs.pkg-config;
            rustup = currentPkgs.rustup;
            rustPlatform = currentPkgs.rustPlatform;
            stdenv = currentPkgs.stdenv;
            webkitgtk_4_1 = currentPkgs.webkitgtk_4_1;
            xdotool = currentPkgs.xdotool;
	    sqlite = currentPkgs.sqlite;
            pkgs = currentPkgs;
          };
          
          rustShell = currentPkgs.callPackage ({
            atkmm,
            cairo,
            gcc,
            gdk-pixbuf,
            glib,
            gtk3,
            mkShell,
            openssl,
            pango,
            pkg-config,
            rustup,
            rustPlatform,
            stdenv,
            webkitgtk_4_1,
            xdotool,
	    sqlite,
            pkgs,
          }:
          let
            overrides = (currentPkgs.lib.importTOML (currentPkgs.lib.readFile ./rust-toolchain.toml));
          in
          mkShell {
            strictDeps = true;
            nativeBuildInputs = [
              gcc
              openssl
              pkg-config
              rustup
              rustPlatform.bindgenHook
            ];
            buildInputs = [
              atkmm
              cairo
              gdk-pixbuf
              glib
              gtk3
              pango
              webkitgtk_4_1
              xdotool
	      sqlite
            ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            shellHook = ''
              export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
              # It's better to use the rustup setup provided by the nixpkg rustup itself
              # But keeping your original logic structure for now:
              export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
            '';
          }) devShellArgs;
          
        in {
          default = rustShell;
        }
      );
    };
}
