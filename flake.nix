{
  description = "A development shell for a Rust project";

  # Define the inputs (dependencies) for your flake
  inputs = {
    # Specify nixpkgs, which is where most packages come from
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11"; # Use a stable channel or your preferred branch
  };

  # Define the outputs of your flake (what it provides)
  outputs = { self, nixpkgs }:
    let
      # Define the systems your flake will support (e.g., x86_64-linux, aarch64-darwin)
      supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];

      # A utility function to apply a function across all supported systems
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      # Import the correct nixpkgs set for each system
      pkgs = forAllSystems (system:
        import nixpkgs {
          inherit system;
          # Configure additional options if needed, e.g., allowUnfree = true;
        }
      );

    in {
      # The main output for a development environment
      devShells = forAllSystems (system:
        let
          currentPkgs = pkgs.${system};
          # The original nix expression logic goes into a separate attribute set
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
            # Since you had `pkgs` in your original expression, we pass the current package set
            pkgs = currentPkgs;
          };
          
          # This is the original logic wrapped in a function call
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
            pkgs, # Added to access pkgs inside the inner function
          }:
          let
            # Assuming rust-toolchain.toml is still in the same directory
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
            ];
            # Use `pkgs` inside the shell definition to access openssl
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            shellHook = ''
              export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
              # It's better to use the rustup setup provided by the nixpkg rustup itself
              # But keeping your original logic structure for now:
              export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
            '';
          }) devShellArgs;
          
        in {
          # The default devShell that users will enter
          default = rustShell;
        }
      );
    };
}
