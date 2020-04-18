use crate::{parsers::parser::MapType, schema::Map};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Osu {
    path: PathBuf,
    map: Map,
    notes: Vec<u32>,
    state: State,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
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

impl MapType for Osu {
    fn init(path: &PathBuf) -> Self {
        Osu {
            path: path.to_path_buf(),
            map: Map::default(),
            notes: vec![],
            state: State::Start,
        }
    }
    fn glob() -> String {
        "**/*.osu".into()
    }
    fn parse_line(&mut self, line: &str) {
        use State::*;
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
                    "AudioFilename" => self.map.audio = self.path.parent().unwrap().join(v).display().to_string(),
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
            Difficulty => {
                let kv = line.split(':').collect::<Vec<_>>();
                let v = kv[1].trim();
                match kv[0] {
                    "CircleSize" => {
                        self.map.keys = v
                            .parse::<f32>()
                            .map_err(|e| format!("Invalid CircleSize: {}", self.path.display()))
                            .unwrap() as i32
                    }
                    _ => (),
                }
            }
            Events => {
                if self.map.background.is_empty() {
                    let e = line.split(',').collect::<Vec<_>>();
                    if e.len() > 2 && e[0] == "0" {
                        self.map.background = self
                            .path
                            .parent()
                            .unwrap()
                            .join(e[2][1..e[2].len() - 1].to_string())
                            .display()
                            .to_string();
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
                // TODO: add support for mania key count
                // TODO: add support for sliders (optionally togglable in-game)
                let ho = line.split(',').collect::<Vec<_>>();
                let x = ho[0].parse::<i32>().expect("Invalid note x coordinate");
                let y = ho[1].parse::<i32>().expect("Invalid note y coordinate");
                let time = ho[2].parse::<i32>().expect("Invalid note time");
                let typ = ho[3].parse::<u8>().expect("Invalid note type");
                // println!("{:08b},{},{},{}",typ,x,y,time);
                // if self.map.mode != "mania" {
                if self.map.notes.0.len() == 0
                    || (self.map.notes.0.len() > 0 && (time - self.map.notes.0[self.map.notes.0.len() - 1].0 as i32) > 10)
                {
                    // TODO: you'll need to actually add duplicates for manias so you cover chords
                    self.map.notes.0.push((time as u32, x as u32));
                }
                // } else {
                //     self.map.notes.0.push((time as u32,x as u32));
                // }
                // if self.map.mode == "mania" && self.map.keys != 4 {
                //     println!("{}",x * self.map.keys/512);
                // }
            }
            _ => (),
        };
    }
    fn get(&mut self) -> Option<Map> {
        if self.map.notes.0.len() < 10 {
            return None;
        }
        // println!("{}",self.map.keys);

        self.map.length = (self.map.notes.0[self.map.notes.0.len() - 1].0 - self.map.notes.0[0].0) as f32 / 1000.0;
        self.map.dmin = 10000;
        let diffs = self
            .map
            .notes
            .0
            .windows(2)
            .map(|pair| pair[1].0 - pair[0].0)
            .collect::<Vec<u32>>();

        self.map.count = diffs.len() as i32 + 1;
        self.map.nps = self.map.count as f32 / self.map.length;

        // deltas
        diffs.iter().for_each(|d| {
            self.map.dmin = std::cmp::min(self.map.dmin, *d as i32);
            self.map.dmax = std::cmp::max(self.map.dmax, *d as i32);
        });
        self.map.davg =
            (self.map.notes.0[self.map.notes.0.len() - 1].0 - self.map.notes.0[0].0) as i32 / (diffs.len() as i32 + 1) as i32;

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
        // TODO: check these if-statements
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
        self.map.difficulty = (1000.0 * self.map.nps * (1.0 / self.map.dmin as f32) * self.map.savg as f32).log2();
        // println!("{}\t{}\t{}\t{}",self.map.difficulty,self.map.nps,self.map.dmin,self.map.savg);

        let mut s = DefaultHasher::new();
        format!(
            "{}{}{}{}{}{}",
            self.map.title, self.map.artist, self.map.creator, self.map.version, self.map.difficulty, self.map.nps
        )
        .hash(&mut s);
        self.map.id = s.finish().to_string();

        Some(self.map.clone())
    }
}
