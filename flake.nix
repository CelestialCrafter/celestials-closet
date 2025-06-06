{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      packages = builtins.listToAttrs (
        builtins.map
          (system: {
            name = system;
            value = {
              default = pkgs.rustPlatform.buildRustPackage {
                pname = "celestials-closet";
                version = "0.1.0";

                src = nixpkgs.lib.cleanSource ./.;
                cargoHash = "sha256-DySaaJ/uqMVqTkvG/tTqozjQ0xjTaBhTyzXs+ePoI50=";
                useFetchCargoVendor = true;

                meta = {
                  description = " my personal website! ";
                  homepage = "https://github.com/celestialCrafter/celestials-closet/";
                  license = nixpkgs.lib.licenses.mpl20;
                  maintainers = [ "CelestialCrafter" ];
                };
              };
            };
          })
          [
            "x86_64-linux"
            "aarch64-linux"
          ]
      );

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          pkg-config
          openssl
        ];
      };
    };
}
