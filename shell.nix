{ pkgs ? import <nixpkgs> { config.allowUnfree = true; } }:
let
  libbass = (pkgs.libbass.overrideAttrs (_: {
    version = "2.4.15";
    src = pkgs.fetchurl {
      url = "https://www.un4seen.com/files/bass24-linux.zip";
      sha256 = "1z01im0l2ydi608vc0n0c1cfyp1vrh6681zn2zkhg98vvs20y805";
    };
  }));
in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    alsaLib
    eudev
    pkgconfig
    libbass
    libbass_fx
    sqlite
    rustup

    # ?
    gcc-unwrapped.lib
    stdenv.cc.cc.lib
    glxinfo
    libGL
    llvm
    llvmPackages.libclang
  ];
  LD_LIBRARY_PATH=
  "${pkgs.llvmPackages.libclang}/lib:"
  +"${pkgs.stdenv.cc.cc.lib}/lib:"
  +"${pkgs.xorg.libX11}/lib:"
  +"${pkgs.xorg.libXcursor}/lib:"
  +"${pkgs.xorg.libXrandr}/lib:"
  +"${pkgs.xorg.libXi}/lib:"
  +"${pkgs.libGL}/lib:"

  # ?
  +"${pkgs.glxinfo}/lib:"
  +"${pkgs.stdenv.cc.cc.lib}/lib:"
  +"${pkgs.alsaLib}/lib:"
  +"${pkgs.gcc-unwrapped.lib}/lib:"
  +"$LD_LIBRARY_PATH";
  shellHook = ''
    echo Welcome to the taipo build environment
  '';
  installPhase = ''
    rustup install stable
    rustup default stable
    rustup component add rust-src
  '';
}

