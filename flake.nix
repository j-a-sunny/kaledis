{
  description = "Kaledis - A new way to LÖVE (Luau + Love2D)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # version + prebuilt-binary URLs/hashes, kept fresh by
        # .github/workflows/update-release-info.yml
        release = import ./nix/kaledis-release.nix;
        version = release.version;
        releaseAssets = release.assets;

        srcPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "kaledis";
          inherit version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl   # git2 / auth-git2 https support
            libgit2   # git2
            libssh2   # git2 ssh support
            zlib      # backhand / flate2
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          # mlua's "vendored" feature builds Luau from C source during the
          # build — that just needs a C compiler, which stdenv provides,
          # no extra buildInput needed.

          # Tests may try to hit the network or a real git remote; disable
          # unless you've checked they're hermetic.
          doCheck = false;

          meta = with pkgs.lib; {
            description = "A new way to LÖVE. Kaledis is a tool for allowing Luau to be used with Love2D via transpiling.";
            homepage = "https://github.com/orpos/kaledis";
            license = licenses.mit;
            mainProgram = "kaledis";
          };
        };

        binPackage =
          let
            asset = releaseAssets.${system} or (throw
              "kaledis: no prebuilt release binary published for ${system}. Use `packages.default` (build from source) instead.");
          in
          pkgs.stdenv.mkDerivation {
            pname = "kaledis-bin";
            inherit version;

            src = pkgs.fetchurl {
              inherit (asset) url;
              sha256 = asset.sha256;
            };

            # The release tarball is a bare `kaledis` binary with no top-level
            # directory, which trips up the default unpack heuristics — just
            # extract it ourselves.
            dontUnpack = true;

            nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.autoPatchelfHook
            ];
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.zlib
              pkgs.stdenv.cc.cc.lib
            ];

            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin
              tar -xzf $src -C $TMPDIR
              install -m755 $TMPDIR/kaledis $out/bin/kaledis
              runHook postInstall
            '';

            meta = with pkgs.lib; {
              description = "A new way to LÖVE (prebuilt upstream release binary, v${version})";
              homepage = "https://github.com/orpos/kaledis";
              license = licenses.mit;
              mainProgram = "kaledis";
              platforms = builtins.attrNames releaseAssets;
            };
          };
      in
      {
        packages = {
          default = srcPackage;
          src = srcPackage;
          bin = binPackage;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ srcPackage ];
          packages = with pkgs; [ rustc cargo rust-analyzer clippy rustfmt ];
        };
      });
}
