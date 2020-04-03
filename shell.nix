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
  ];
  LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
  shellHook = ''
    echo Hello
  '';
  installPhase = ''
    rustup install stable
    rustup default stable
  '';
}

