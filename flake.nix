{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    {
      packages = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          program = pkgs.rustPlatform.buildRustPackage {
            pname = "celestials-closet";
            version = "0.1.0";
            src = nixpkgs.lib.cleanSource ./.;
            cargoHash = "sha256-snotgIjEG4mRljU9WR41qPnThpVPGL3L12HLOpn5vDI=";
            meta = {
              description = "my personal website!";
              homepage = "https://github.com/celestialCrafter/celestials-closet/";
              license = pkgs.lib.licenses.mpl20;
              maintainers = [ "CelestialCrafter" ];
            };
          };

          container = pkgs.dockerTools.buildImage {
            name = program.pname;
            tag = "latest";

            config = {
              Cmd = [ "${program}/bin/celestials-closet" ];
              ExposedPorts."80/tcp" = { };
              Volumes."/data/" = { };
              WorkingDir = "/data";
            };
          };
        in
        {
          inherit container;
          default = program;
        }
      );

      devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
        packages = with nixpkgs.legacyPackages.x86_64-linux; [
          rustc
          cargo
          pkg-config
          openssl
        ];
      };
    };
}
