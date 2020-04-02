# taipo
A rhythm game written in Rust (using SDL2, libbass, and sqlite).  

## Features
### Variable speed, configurable binds,
### Parses and converts the following 3 formats into a separate standard:  
1) .osu  
2) .sm  
3) .bms  
### Search, sort, filter, and group maps/scores/collections using SQL statements and dozens of fields
Ex: seach for all maps beween 180 and 220 bpm with less than 5 nps and a top score with less than 97% accuracy. 
`map.bpm between 180 and 220 and map.nps>5 and score.acc<97`

## Install
### Package
taipo can be installed via most package managers: apt, yum, pacman, xbps, snap, etc.
### Releases
taipo can be installed via the release page: https://github.com/notgate/taipo/releases
### Steam
taipo can be installed via steam: https://store.steampowered.com/app/{id}/taipo 
### Source 
taipo can be built from source:
```bash
$ nix-shell
cargo build --release
export PATH="$PWD/target/release:$PATH"
taipo
```
