{ pkgs ? import <nixpkgs> { config.allowUnfree = true; }}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    libbass
    cargo
    rustup
    #libbass_fx
    #(import ./libbass.nix)
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
  installPhase = ''
    rustup install stable
    rustup default stable
  '';
}

