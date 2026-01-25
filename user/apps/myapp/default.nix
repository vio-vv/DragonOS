{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  pname = "helloworld";
  version = "0.1.0";

  src = ./.;

  makeFlags = [
    "ARCH=x86_64"
    "CROSS_COMPILE=${pkgs.stdenv.cc.targetPrefix}"
  ];

  installPhase = ''
    mkdir -p $out/bin
    install -m755 helloworld $out/bin/helloworld
  '';

  meta = {
    description = "一个用来测试helloworld的app";
    platforms = [ "x86_64-linux" ];
  };
}
