pub trait MusicPlayer {
    fn init() -> MusicPlayer;
    fn free(&mut self);

    fn load(&mut self, file: &str);

    fn set_pos(&mut self, val: f64);
    fn get_pos(&self) -> f64;
    fn set_playing(&mut self, val: bool);
    fn get_playing(&self) -> f64;
    fn set_looping(&mut self, val: bool);
    fn get_looping(&self) -> bool;
    fn set_speed(&mut self, val: f32);
    fn get_speed(&self) -> f32;
    fn set_volume(&mut self, val: f32);
    fn get_volume(&self) -> f32;
}

pub struct BassPlayer {
    handle: u32,
    buffer: Vec<u8>,
    pub playing: bool,
    pub looping: bool,
    pub speed: f32,
    pub volume: f32,
}

impl MusicPlayer for BassPlayer {
    pub fn init() -> BassPlayer {
        unsafe { BASS_Init(-1, 44100, 0, 0, 0) };
        BassPlayer {
            handle: 0,
            buffer: vec![],
            playing: false,
            looping: false,
            speed: 0.0,
            vol: 0.0,
            from: 0.0,
            to: 0.0,
        }
    }
    pub fn free(&mut self) {
        unsafe { BASS_Free() };
    }
    fn load(&mut self, path: &str) {
        if self.handle != 0 {
            unsafe { BASS_StreamFree(self.handle) };
        }

        let mut audio_file = std::fs::File::open(path).unwrap();
        self.buffer = Vec::new();
        audio_file.read_to_end(&mut self.buffer).unwrap();

        self.handle = unsafe {
            let ptr = self.buffer.as_ptr();
            let chan = BASS_StreamCreateFile(1, ptr, 0, self.buffer.len() as u64, 0x200000); //BASS_STREAM_DECODE
            BASS_FX_TempoCreate(chan, 0x10000 | 4) //BASS_FX_FREESOURCE|BASS_SAMPLE_LOOP
        };
    }
}
