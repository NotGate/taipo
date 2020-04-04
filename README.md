A rhythm game written in Rust

## Features (to be implemented ...)
* Variable speed, configurable settings, and advanced queries  
* Parses and converts `osu`, `sm`, `bms`, and `ojn` maps into a unifying format  
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

## Useful Links
[kanban](https://github.com/NotGate/taipo/projects/1?fullscreen=true)  
[osu file format](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Osu_(file_format))  
[sm file format](https://github.com/stepmania/stepmania/wiki/sm)  
[ssc file format](https://github.com/stepmania/stepmania/wiki/ssc)  
[bms file format](https://en.wikipedia.org/wiki/Be-Music_Source)  
[ojn file format](https://fileformats.fandom.com/wiki/O2Jam_note_files)  
[osu db format](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Db_(file_format))  
[libbass docs](http://www.un4seen.com/doc/#bass/bass.html)  
[sdl2 docs](https://rust-sdl2.github.io/rust-sdl2/sdl2/)  
[rusqlite docs](https://docs.rs/rusqlite/0.21.0/rusqlite/)  
Also, make sure to reference open-source games with parsers, like Etterna, McOsu, open2jam, etc.