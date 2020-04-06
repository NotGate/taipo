use crate::{parsers::parser::FSM, schema::Map};

#[derive(Debug)]
pub struct OsuFsm {
    map: Map,
    state: OsuState,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OsuState {
    Start,
    General,
    Metadata,
    Editor,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects,
    End,
}

impl FSM for OsuFsm {
    fn init() -> Self {
        OsuFsm {
            map: Map::default(),
            state: OsuState::Start,
        }
    }
    fn glob() -> String {
        "**/*.osu".into()
    }
    fn parse_line(&mut self, line: &str) {
        use OsuState::*;
        let state = match line {
            "[General]" => General,
            "[Metadata]" => Metadata,
            "[Editor]" => Editor,
            "[Difficulty]" => Difficulty,
            "[Events]" => Events,
            "[TimingPoints]" => TimingPoints,
            "[Colours]" => Colours,
            "[HitObjects]" => HitObjects,
            _ => self.state,
        };
        if state != self.state {
            self.state = state;
            return;
        }
        match self.state {
            Start => {
                self.map.source = "osu".into();
                let s = line.split(' ').collect::<Vec<_>>();
                self.map.format = s[s.len()-1].into();
            }
            General => {
                let kv = line.split(':').collect::<Vec<_>>();
                let v = kv[1].trim();
                match kv[0] {
                    "AudioFilename" => self.map.audio = v.into(),
                    "PreviewTime" => self.map.preview = v.parse::<f32>().expect("Invalid PreviewTime") / 1000.0,
                    "Mode" => {
                        self.map.mode = match v.parse::<i32>().expect("Invalid Mode") {
                            1 => "taiko",
                            3 => "mania", // I don't know how many keys there are yet
                            0 | 2 | _ => "other",
                        }
                        .into()
                    }
                    _ => (),
                }
            }
            Metadata => {
                let kv = line.split(':').collect::<Vec<_>>();
                let v = kv[1].trim();
                match kv[0] {
                    "Artist" => self.map.artist = v.into(),
                    "Title" => self.map.title = v.into(),
                    "Creator" => self.map.creator = v.into(),
                    "Version" => self.map.version = v.into(),
                    "Tags" => self.map.tags = v.into(),
                    _ => (),
                }
            }
            Events => {
                let e = line.split(',').collect::<Vec<_>>();
                if e.len() > 2 && e[0] == "0" {
                    // bg offset are e[3] and e[4]
                    self.map.background = e[2][1..e[2].len() - 2].into();
                }
            }
            TimingPoints => {
                let tp = line.split(',').collect::<Vec<_>>();
                self.map.bpm = 60000.0 / tp[1].parse::<f32>().expect("Invalid beatLength");
            }
            HitObjects => {
                // x,y,time,type,hitSound,objectParams,hitSample
            }
            _ => (),
        };
    }
    fn get(&self) -> Map {
        self.map.clone()
    }
}
