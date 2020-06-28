{
  description = "nix written in rust";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

      # Memoize nixpkgs for different platforms for efficiency.
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        });

      commondDeps = pkgs: {
        buildDeps = with pkgs;
          [

            # TODO: tests
          ];

        propBuildDeps = with pkgs; [ sqlite ];
      };

      version = "0.1.0"; # TODO: version managemant

    in {
      overlay = final: prev: {
        rusty-nix = with final;
          with commondDeps pkgs;
          (rustPlatform.buildRustPackage {
            pname = "rusty-nix";
            inherit version;

            src = self;

            outputs = [ "out" "doc" ]; # TODO: dev/doc?

            buildInputs = buildDeps;

            propagatedBuildInputs = propBuildDeps;

            cargoSha256 =
              "7059c0be4492cd0531e97ccdfcdeafdd813406b0e60165fa25de0707fcf3360b";

            postInstall = ''
              cargo doc --workspace --release --all-features --frozen --offline --target-dir $doc

              mkdir -p $doc/nix-support/
              echo "doc manual $doc/" >> $doc/nix-support/hydra-build-products
            '';

          });
      };

      hydraJobs = {
        build =
          nixpkgs.lib.genAttrs systems (system: nixpkgsFor.${system}.rusty-nix);
      };

      packages =
        forAllSystems (system: { inherit (nixpkgsFor.${system}) rusty-nix; });

      defaultPackage =
        forAllSystems (system: self.packages.${system}.rusty-nix);
    };
}
