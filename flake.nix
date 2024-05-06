{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
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
                  git-cliff
                  tailwindcss
                  rustc
                  cargo
                  rust-analyzer
                ]);

              services.postgres = {
                enable = true;
                package = pkgs.postgresql_15;
                initialDatabases = [{name = "rentirement";}];
                extensions = extensions: [];
              };

              processes = {
                server.exec = "watchexec --restart --exts go,templ -- go run main.go";
                tailwind.exec = "watchexec --restart --exts go,css,temple -- tailwindcss -i input.css -o assets/style.css";
                templ.exec = "watchexec --restart --exts templ -- templ generate";
              };

              enterShell = ''
              '';
            }
          ];
        };
      });
  };
}
