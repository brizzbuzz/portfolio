{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    crane,
    nixpkgs,
    nixpkgs-unstable,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (system: {
      devenv-up = self.devShells.${system}.default.config.procfileScript;
    });

    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              packages =
                (with pkgs; [
                  atlas
                  watchexec
                  xh
                ])
                ++ (with pkgs-unstable; [
                  cargo
                  git-cliff
                  htmx-lsp
                  nil
                  rustc
                  rustfmt
                  rust-analyzer
                  tailwindcss
                  tailwindcss-language-server
                  vscode-langservers-extracted
                ]);

              services.postgres = {
                enable = true;
                package = pkgs.postgresql_15;
                initialDatabases = [{name = "portfolio";}];
                extensions = extensions: [];
              };

              processes = {
                server.exec = "cargo watch -x run";
                tailwind.exec = "watchexec --restart --exts css,html -- tailwindcss -i input.css -o public/styles/tailwind.css";
              };

              enterShell = ''
              '';
            }
          ];
        };
      });
  };
}
