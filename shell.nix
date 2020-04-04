{ pkgs ? import <nixpkgs> { config.allowUnfree = true; }}:
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
    alsa-firmware
    alsaLib
    alsaOss
    alsaPlugins
    alsaTools
    alsaUtils
    libpulseaudio
    cargo
    rustup
    libbass
    libbass_fx
    gcc-unwrapped.lib
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    glxinfo
    libGL
    pkgconfig
    alsaLib
    eudev
    stdenv.cc.cc.lib
    # examples
    gtk2
    xfce.libglade
    # dev
    gdb
    rr
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
    +"$LD_LIBRARY_PATH";
  shellHook = ''
    echo Hello
  '';
  installPhase = ''
    rustup install stable
    rustup default stable
  '';
}

