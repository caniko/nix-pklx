{
  description = "nix-pklx — Pkl ↔ Nix interop: evaluate Pkl files to native Nix expressions + toPkl/fromPkl Nix library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    harbor = {
      url = "git+https://github.com/caniko/harbor.git?ref=feat/harbor-monorepo-components&rev=e8b1996580e4ecd51dd402edf4a16d9df88e3a1b";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.crane.follows = "crane";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    plinth = {
      # Only the deployment library is consumed. Retain its locked revision
      # without evaluating Plinth's recursive documentation input graph.
      url = "git+https://github.com/caniko/plinth.git?ref=trunk&rev=e9951aeb17ce50b92b4e50a5abe4c25516c86060";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rust-overlay,
    harbor,
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
    projectSiteFor = system:
      import (plinth.outPath + "/nix/project-site.nix") {
        pkgs = pkgsFor system;
        inherit (nixpkgs) lib;
      };
    treefmtFor = system:
      harbor.inputs.treefmt-nix.lib.evalModule (pkgsFor system) {
        imports = [harbor.treefmtModules.core-nix harbor.treefmtModules.core-toml harbor.treefmtModules.rust-rust];
        projectRootFile = "flake.nix";
        programs.rustfmt.package =
          (harbor.lib.rust.mkToolchain {
            pkgs = pkgsFor system;
            toolchainProfile = "nightly";
          }).rustToolchain;
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
      deploy-pages = (projectSiteFor system).mkDeployPagesApp {
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
        to-pkl-roundtrip = let
          fixture = import ./tests/to-pkl.nix {inherit (pkgs) lib;};
        in
          pkgs.runCommand "to-pkl-roundtrip" {
            nativeBuildInputs = [pkgs.pkl pkgs.jq pkgs.nix self.packages.${system}.pklx];
            SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          } ''
            pkl eval -f json ${pkgs.writeText "roundtrip.pkl" fixture.document} | jq -S . > actual.json
            jq -S . ${pkgs.writeText "expected.json" (builtins.toJSON fixture.expected)} > expected.json
            diff -u expected.json actual.json
            pklx eval ${pkgs.writeText "native-roundtrip.pkl" fixture.nativeDocument} -o native.nix
            nix-instantiate --store dummy:// --eval --strict --json native.nix | jq -S . > native.json
            jq -S . ${pkgs.writeText "native-expected.json" (builtins.toJSON fixture.nativeExpected)} > native-expected.json
            diff -u native-expected.json native.json
            touch "$out"
          '';

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

        pklx-fmt = (treefmtFor system).config.build.check self;

        # Fail if flake inputs ever point at the retired Codeberg/Codefloe
        # mirrors again (fleet migrated to github.com/caniko/*).
        # sourceUrl package metadata is excluded: informational only, not fetched.
        host-pinning = let
          # Split across literals so this file never matches its own pattern.
          staleHosts = "cod" + "eberg|cod" + "efloe";
        in
          pkgs.runCommand "nix-pklx-host-pinning" {} ''
            if ${pkgs.lib.getExe pkgs.ripgrep} -v "sourceUrl" ${./flake.nix} ${./flake.lock} \
              | ${pkgs.lib.getExe pkgs.ripgrep} -q "${staleHosts}"; then
              echo "ERROR: retired forge host in flake inputs:" >&2
              ${pkgs.lib.getExe pkgs.ripgrep} -v "sourceUrl" ${./flake.nix} ${./flake.lock} \
                | ${pkgs.lib.getExe pkgs.ripgrep} -n "${staleHosts}" >&2 || true
              exit 1
            fi
            touch $out
          '';
      }
    );

    devShells = forSystems (
      system: let
        pkgs = pkgsFor system;
        toolchain = harbor.lib.rust.mkToolchain {
          inherit pkgs;
          toolchainProfile = "nightly";
        };
        cargoConfig = harbor.lib.rust.mkCargoConfig {inherit pkgs;};
        cross = harbor.lib.rust.mkCross {inherit pkgs system;};
      in
        (harbor.lib.rust.mkDevShells {
          inherit pkgs cross cargoConfig;
          inherit (toolchain) craneLib;
          packages = [pkgs.mdbook (treefmtFor system).config.build.wrapper];
        })
        // {
          docs = harbor.lib.rust.mkDocsShell {
            inherit pkgs cross cargoConfig;
            inherit (toolchain) craneLib;
            packages = [pkgs.mdbook (treefmtFor system).config.build.wrapper];
            extraShellHook = ''
              echo "Documentation: mdbook serve docs"
            '';
          };
        }
    );
    formatter = forSystems (system: (treefmtFor system).config.build.wrapper);
  };
}
