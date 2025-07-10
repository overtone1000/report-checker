# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [
        llvmPackages.bintools
        #lldb #Should be available through vscode extension
        clang
        cargo
        rustc
        rustup
    ];

    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}