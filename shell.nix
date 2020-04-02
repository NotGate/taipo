with import <nixpkgs> { config.allowUnfree = true; };
stdenv.mkDerivation {
  name = "build-environment";
  buildInputs = with pkgs; [
    cargo
    #libbass
    #libbass_fx
    #gcc-unwrapped.lib
    #xorg.libX11
    #xorg.libXcursor
    ##xorg.libXrandr
    #xorg.libXi
    #glxinfo
    #libGL
  ];
  shellHook = ''
		echo Hello
  '';
}
