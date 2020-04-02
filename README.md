A rhythm game written in Rust

## Features
* Variable speed, configurable settings, and advanced queries  
* Parses and converts `osu`, `sm`, and `bms` maps into a unifying format  
* Search, sort, filter, and group maps/scores/collections using SQL statements with dozens of available fields  
`map.bpm between 180 and 220 and map.nps>5 and score.acc<97 and collection.name="practice"`  

## Install
*Package*: apt, yum, pacman, xbps, snap  
*Release*: https://github.com/notgate/taipo/releases  
*Steam*: https://store.steampowered.com/app/{id}/taipo  
*Source*: 
```bash
$ nix-shell
cargo build --release
export PATH="$PWD/target/release:$PATH" # source this somewhere
taipo
```
