{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      inherit (pkgs) lib;

      craneLib = crane.mkLib pkgs;

      src = lib.cleanSourceWith {
        src = ./.; # The original, unfiltered source
        filter = path: type:
          (craneLib.filterCargoSources path type)
          || (lib.hasInfix "templates/" path)
          || (lib.hasInfix "public/" path);
      };

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs =
          [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

        # Additional environment variables can be set directly
        # MY_CUSTOM_VAR = "some value";
      };

      craneLibLLvmTools =
        craneLib.overrideToolchain
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "llvm-tools"
          "rustc"
          "rust-analyzer"
        ]);

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      my-crate = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });

      docker-image = pkgs.dockerTools.streamLayeredImage {
        name = "portfolio";
        tag = "latest"; # TODO: How to version properly?
        created = "now";
        contents = [
          my-crate
          ./public
          ./templates
        ];
        config = {
          Cmd = ["${my-crate}/bin/portfolio"];
          Env = [
            "ASSET_PATH=/"
            "ROCKET_ADDRESS=0.0.0.0"
            "ROCKET_ENV=production"  # TODO: How to set this dynamically?
          ];
        };
      };
    in {
      checks = {
        inherit my-crate;
        my-crate-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        # my-crate-doc = craneLib.cargoDoc (commonArgs
        #   // {
        #     inherit cargoArtifacts;
        #   });

        my-crate-fmt = craneLib.cargoFmt {
          inherit src;
        };

        # Audit dependencies
        my-crate-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        my-crate-deny = craneLib.cargoDeny {
          inherit src;
        };

        my-crate-nextest = craneLib.cargoNextest (commonArgs
          // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
      };

      packages =
        {
          inherit docker-image;
          default = my-crate;
        }
        // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs
            // {
              inherit cargoArtifacts;
            });
        };

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          bacon # Backround Rust Runner
          cargo-watch # Rust hot-reloading
          flyctl # Fly.io CLI
          just # Justfile runner
          postgresql_16 # PostgreSQL database
          process-compose # Process management tool
          tailwindcss # Tailwind CSS Utility CLI
          watchexec # File watcher for non-cargo processes
        ];
      };
    });
}
