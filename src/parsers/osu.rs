use crate::{parsers::parser::FSM, schema::Map};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct OsuFsm {
    path: PathBuf,
    notes: Vec<i32>,
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
    fn init(path: &PathBuf) -> Self {
        OsuFsm {
            path: path.to_path_buf(),
            notes: vec![],
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
                self.map.format = s[s.len() - 1].into();
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
                if self.map.background.is_empty() {
                    let e = line.split(',').collect::<Vec<_>>();
                    if e.len() > 2 && e[0] == "0" {
                        // bg offset are e[3] and e[4]
                        self.map.background = e[2][1..e[2].len() - 2].into();
                    }
                }
            }
            TimingPoints => {
                if self.map.bpm == 0.0 {
                    let tp = line.split(',').collect::<Vec<_>>();
                    let bpm = 60000.0 / tp[1].parse::<f32>().expect("Invalid beatLength");
                    if bpm > 0.0 {
                        self.map.bpm = bpm;
                    }
                }
            }
            HitObjects => {
                let ho = line.split(',').collect::<Vec<_>>();
                let x = ho[0].parse::<i32>().expect("Invalid note x coordinate");
                let y = ho[1].parse::<i32>().expect("Invalid note y coordinate");
                let time = ho[2].parse::<i32>().expect("Invalid note time");
                let typ = ho[3].parse::<u8>().expect("Invalid note type");
                // println!("{:08b},{},{},{}",typ,x,y,time);

                // should I filter this hear instead of during get()?
                if self.notes.len() == 0 || (self.notes.len() > 0 && (time - self.notes[self.notes.len() - 1]) > 10) {
                    self.notes.push(time);
                }
            }
            _ => (),
        };
    }
    fn get(&mut self) -> Option<Map> {
        if self.notes.len() < 10 {
            return None;
        }

        self.map.length = (self.notes[self.notes.len() - 1] - self.notes[0]) as f32 / 1000.0;
        self.map.dmin = 10000;
        let diffs = self
            .notes
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            // figure out why there are negative deltas
            // .filter(|v| *v > 10)
            .collect::<Vec<i32>>();

        // This shouldn't happen if you filter ahead of time
        if diffs.len() == 0 {
            return None;
        }

        self.map.count = diffs.len() as i32 + 1;
        self.map.nps = self.map.count as f32 / self.map.length;

        // deltas
        diffs.iter().for_each(|d| {
            self.map.dmin = std::cmp::min(self.map.dmin, *d);
            self.map.dmax = std::cmp::max(self.map.dmax, *d);
            self.map.davg += d;
        });
        self.map.davg /= diffs.len() as i32 + 1;

        // I can do it like this if I don't filter diffs? idk
        // self.map.davg = (self.map.notes[self.map.notes.len() - 1] - self.map.notes[0]) / (diffs.len() as i32 + 1);

        // streaks
        let m = self.map.dmin as f32;
        let mut streak = 0;
        let mut streaks: Vec<i32> = vec![];
        diffs.iter().for_each(|d| {
            let d = *d as f32;
            if d < m * 1.2 {
                streak += 1;
            } else if streak != 0 {
                streaks.push(streak as i32 + 1);
            } else {
                streak = 0;
            }
        });
        if streak != 0 {
            streaks.push(streak as i32 + 1);
        }
        // How is this possible?
        if streaks.len() == 0 {
            return None;
        }
        self.map.smin = 10000;
        streaks.iter().for_each(|d| {
            self.map.smin = std::cmp::min(self.map.smin, *d);
            self.map.smax = std::cmp::max(self.map.smax, *d);
            self.map.savg += d;
        });
        self.map.savg /= streaks.len() as i32;

        // difficulty
        self.map.difficulty = (1000.0 * self.map.nps * (1.0 / self.map.dmin as f32) * self.map.savg as f32).log2();

        // set map notes to a compressed string version
        // self.map.notes = diffs;

        // println!("{}\t{}\t{}\t{}",self.map.difficulty,self.map.nps,self.map.dmin,self.map.savg);

        let mut s = DefaultHasher::new();
        format!(
            "{}{}{}{}{}{}",
            self.map.title, self.map.artist, self.map.creator, self.map.version, self.map.difficulty, self.map.nps
        )
        .hash(&mut s);
        self.map.id = s.finish().to_string();
        // println!("{}\t{}", self.map.id,self.map.nps);

        Some(self.map.clone())
    }
}
