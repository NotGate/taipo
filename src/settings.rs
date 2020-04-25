use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    // internal settings
    pub version: String,
    pub query: String,
    pub parse_date: u64, // date the last map parse was performed (if any folders are newer than that default "", reparse)

    // gameplay settings
    pub mode: String, // last selected mode (other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k)
    pub seed: u64,
    pub speed: f32,
    pub volume: f32,
    pub aset: i32, // should only ever be negative (play audio sooner) (= -mp.latency() by default)
    pub iset: i32, // should only ever be negative (substract from timestamp)
    pub window: i32,

    // game settings
    pub skin: String,
    pub font: String,
    pub w: usize,
    pub h: usize,
    pub borderless: bool,
    pub maximized: bool,
    pub window_mode: String,
    pub bindings: HashMap<String, Vec<u64>>,
}

impl Settings {
    pub fn init() -> Result<Settings, String> {
        let default = Settings {
            version: "1.0.0".into(),
            query: "m.dmin between 80 and 260 and m.nps>3.2 order by m.dmin desc, m.nps asc".into(), //and s.acc>0.9
            parse_date: Utc::now().timestamp() as u64,

            mode: "4k".into(),
            seed: 0,
            speed: 1.0,
            volume: 0.05,
            aset: 0,
            iset: -60,
            window: 100,

            skin: "".into(),
            font: "".into(),
            w: 1280,
            h: 720,
            borderless: true,
            maximized: false,
            window_mode: "".into(),
            bindings: HashMap::new(),
        };

        let s = String::from_utf8(std::fs::read("settings.json").map_or(vec![], |v| v))
            .map_err(|e| format!("Could not convert file contents to string: {}", e))?;
        println!("loaded: {}", s);

        // TODO: golf this
        if let Ok(v) = serde_json::from_str(&s) {
            Ok(v)
        } else {
            Ok(default)
        }
    }
    pub fn save(&mut self) -> Result<(), String> {
        let s = serde_json::to_string(self).map_err(|e| format!("Could not convert Settings to String: {}", e))?;
        println!("saved: {}", s);
        std::fs::write("settings.json", s).map_err(|e| format!("Could not write to file: {}", e))
    }
}
