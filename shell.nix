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
    # rust
    cargo
    rustup
    # audio
    alsaLib
    libbass
    libbass_fx
    # sqlite3
    sqlite
    # gcc, rust
    gcc-unwrapped.lib
    pkgconfig
    eudev
    stdenv.cc.cc.lib
    # graphics
    SDL2
    SDL2_ttf
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    glxinfo
    libGL
    # bindgen
    llvm
    llvmPackages.libclang
    # dev
    # gdb
    # rr
  ];
  LD_LIBRARY_PATH=
  "${pkgs.stdenv.cc.cc.lib}/lib:"
  +"${pkgs.alsaLib}/lib:"
  +"${pkgs.gcc-unwrapped.lib}/lib:"
  +"${pkgs.xorg.libX11}/lib:"
  +"${pkgs.xorg.libXcursor}/lib:"
  +"${pkgs.xorg.libXrandr}/lib:"
  +"${pkgs.xorg.libXi}/lib:"
  +"${pkgs.glxinfo}/lib:"
  +"${pkgs.libGL}/lib:"
  +"${pkgs.llvmPackages.libclang}/lib:"
  +"$LD_LIBRARY_PATH";
  shellHook = ''
    echo Welcome to my shell.nix uwu
  '';
  installPhase = ''
    rustup install stable
    rustup default stable
    rustup component add rust-src
  '';
}

