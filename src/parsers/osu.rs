use crate::{parsers::parser::FSM, schema::Map};

#[derive(Debug)]
pub struct OsuFsm {
    map: Map,
    state: OsuState,
}

#[derive(Debug, std::cmp::PartialEq)]
pub enum OsuState {
    Start,
    General,      //kv
    Metadata,     //kv
    Editor,       //kv
    Difficulty,   //kv
    Events,       //Comma-separated lists
    TimingPoints, //Comma-separated lists
    Colours,      //kv
    HitObjects,   //Comma-separated lists
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
        if self.state == Start {
            self.map.nps = 3.0;
        }
        self.state = match self.state {
            Start => General,
            General => Metadata,
            Metadata => Editor,
            Editor => Difficulty,
            Difficulty => Events,
            Events => TimingPoints,
            TimingPoints => Colours,
            Colours => HitObjects,
            HitObjects => End,
            End => End,
        };
        if self.state != End {
            println!("{:?},{}", self.state, line);
        }
    }
    fn get(&self) -> Map {
        self.map.clone()
    }
}
