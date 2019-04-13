with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "libsecp256k1-env";
  buildInputs = [
    gcc rustc cargo gdb openssl pkgconfig
  ];
}
