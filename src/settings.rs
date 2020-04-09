use std::collections::HashMap;

// TODO: implement Settings {de}serialization
// add defaults for everything
// remember to preset aset somewhere with mp.get_delay
pub struct Settings {
    // internal settings
    version: String, // taipo version
    query: String,   // last sql query
    parse_date: u64, // date the last map parse was performed (if any folders are newer than that default "", reparse)

    // gameplay settings
    mode: String, // last selected mode (other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k)
    seed: u64,    // last selected seed
    speed: f32,   // last selected speed
    volume: f32,  // last selected volume
    aset: f32, // last selected audio offset (s) - should only ever be negative (play audio sooner) (= -mp.latency() by default)
    iset: f32, // last selected input offset (s) - should only ever be negative (substract from timestamp)
    window: f32, // last selected hit window (s)

    // game settings
    skin: String,
    font: String, // Font
    resolution: (f32, f32),
    window_mode: String,                 // String -> SDL
    bindings: HashMap<String, Vec<u64>>, // u64 -> SDL_Input
}
