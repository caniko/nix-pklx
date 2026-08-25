{
  description = "nix-pklx — Pkl ↔ Nix interop: evaluate Pkl files to native Nix expressions + toPkl/fromPkl Nix library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rs-harbor = {
      url = "git+ssh://git@github.com/caniko/harbor-rs.git?ref=trunk&rev=05cc4f162b55fa904b687db1821e2463fa813e50";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.crane.follows = "crane";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    plinth = {
      url = "git+ssh://git@github.com/caniko/plinth.git";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rs-harbor.follows = "rs-harbor";
      inputs.nix-cache-pin.inputs.rs-harbor.follows = "rs-harbor";
      inputs.nix-pklx.url = "git+https://github.com/caniko/nix-pklx.git?ref=trunk&rev=541c3655e9251fdd047f96a4f30810fa21f89d2f";
      inputs.nix-pklx.inputs.rs-harbor.follows = "rs-harbor";
      inputs.nix-pklx.inputs.plinth.follows = "plinth";
      inputs.nix-pklx.inputs.plinth.inputs.rs-harbor.follows = "rs-harbor";
      inputs.nix-pklx.inputs.plinth.inputs.nix-cache-pin.inputs.rs-harbor.follows = "rs-harbor";
      inputs.nix-pklx.inputs.plinth.inputs.nix-pklx.inputs.rs-harbor.follows = "rs-harbor";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rust-overlay,
    rs-harbor,
    plinth,
    ...
  }: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
    forSystems = nixpkgs.lib.genAttrs supportedSystems;

    pkgsFor = system:
      import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
  in {
    lib = forSystems (system:
      import ./lib {
        inherit (nixpkgs) lib;
        pkgs = pkgsFor system;
        pklx = self.packages.${system}.pklx;
      });

    apps = forSystems (system: {
      eval = {
        type = "app";
        program = "${self.packages.${system}.pklx}/bin/pklx";
      };
      default = self.apps.${system}.eval;
      deploy-pages = plinth.lib.${system}.mkDeployPagesApp {
        domain = "nix-pklx.tartanoglu.com";
      };
    });

    packages = forSystems (
      system: let
        pkgs = pkgsFor system;
        craneLib = crane.mkLib pkgs;

        docs = pkgs.stdenv.mkDerivation {
          pname = "nix-pklx-docs";
          version = "0.1.0";
          src = ./docs;
          nativeBuildInputs = [pkgs.mdbook];
          buildPhase = ''
            mdbook build "$src" -d "$TMPDIR/book"
          '';
          installPhase = ''
            cp -r "$TMPDIR/book" "$out"
          '';
        };
      in {
        default = craneLib.buildPackage {
          pname = "pklx";
          version = "0.1.0";
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          doCheck = true;
        };
        pklx = self.packages.${system}.default;
        inherit docs;
        site = pkgs.runCommand "nix-pklx-site" {} ''
          mkdir -p $out
          cp -rL --no-preserve=mode ${docs}/. $out/
          printf '%s\n' "nix-pklx.tartanoglu.com" > $out/.domains
        '';
      }
    );

    checks = forSystems (
      system: let
        pkgs = pkgsFor system;
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          pname = "pklx";
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        pklx-tests = craneLib.cargoTest (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoExtraArgs = "--all-features";
          }
        );

        pklx-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoExtraArgs = "--all-features";
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          }
        );

        pklx-fmt = craneLib.cargoFmt {
          inherit src;
          pname = "pklx";
        };
      }
    );

    devShells = forSystems (
      system: let
        pkgs = pkgsFor system;
        toolchain = rs-harbor.lib.mkToolchain {inherit pkgs; toolchainProfile = "nightly";};
        cargoConfig = rs-harbor.lib.mkCargoConfig {inherit pkgs;};
        cross = rs-harbor.lib.mkCross {inherit pkgs system;};
      in
        (rs-harbor.lib.mkDevShells {
          inherit pkgs cross cargoConfig;
          inherit (toolchain) craneLib;
          packages = with pkgs; [mdbook];
        })
        // {
          docs = rs-harbor.lib.mkDocsShell {
            inherit pkgs cross cargoConfig;
            inherit (toolchain) craneLib;
            packages = with pkgs; [mdbook];
            extraShellHook = ''
              echo "Documentation: mdbook serve docs"
            '';
          };
        }
    );
  };
}
