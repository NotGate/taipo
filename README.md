A rhythm game written in Rust

## Features (to be implemented ...)
* Variable speed, configurable settings, and advanced queries  
* Parses and converts `osu`, `sm`, and `bms` maps into a unifying format  
* Search, sort, filter, and group maps/scores/collections using SQL statements with dozens of available fields  
`map.bpm between 180 and 220 and map.nps>5 and score.acc<97 and collection.name="practice"`  
* Multiple modes: taiko-typing, polar 10K, generic VSRG

## Install (to be distributed ...)
* *Package*: apt, yum, pacman, xbps, snap  
* *Release*: https://github.com/notgate/taipo/releases  
* *Steam*: https://store.steampowered.com/app/{id}/taipo  
* *Source*: 
```bash
git clone https://github.com/notgate/taipo
cd taipo
nix-shell
cargo run --release
```
