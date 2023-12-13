let
  pkgs = import <nixpkgs> {};
in
  pkgs.stdenv.mkDerivation {
    name = "my-package";
    src = ./.;
    buildInputs = [
      pkgs.rustc
      pkgs.cargo
    ];

    LIBCLANG_PATH = "${pkgs.llvmPackages_11.libclang.lib}/lib"; 
  }


