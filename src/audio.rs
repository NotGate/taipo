#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fs::File, io::Read, ptr};
include!("bass/wrapper.rs");
include!("bass/bindings.rs");

pub struct MusicPlayer {
    handle: u32,
    buffer: Vec<u8>,
    pub playing: bool,
    pub looping: bool,
    pub speed: f32,
    pub volume: f32,
}

impl MusicPlayer {
    pub fn init() -> Result<MusicPlayer, String> {
        Bass::init(44100, 0)?;
        Ok(MusicPlayer {
            handle: 0,
            buffer: vec![],
            playing: false,
            looping: false,
            speed: 1.0,
            volume: 0.5,
        })
    }
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        if self.handle != 0 {
            Bass::stream_free(self.handle)?;
        }
        let mut f = File::open(path).expect("Could not open audio file");
        self.buffer = Vec::new();
        f.read_to_end(&mut self.buffer).expect("Could not read audio file");
        self.handle = Bass::fx_tempo_create(
            Bass::stream_create_file(&self.buffer, BASS_STREAM_DECODE)?,
            BASS_FX_FREESOURCE | BASS_MUSIC_LOOP,
        )?;
        Ok(())
    }
    pub fn seek(&mut self, pos: f64) -> Result<(), String> {
        Bass::channel_set_position(self.handle, Bass::channel_seconds2bytes(self.handle, pos)?)
    }
    pub fn pos(&self) -> Result<f64, String> {
        Bass::channel_bytes2seconds(self.handle, Bass::channel_get_position(self.handle)?)
    }
    pub fn play(&mut self) -> Result<(), String> {
        Bass::channel_play(self.handle, 0)
    }
    pub fn pause(&mut self) -> Result<(), String> {
        Bass::channel_pause(self.handle)
    }
    pub fn set_speed(&mut self, val: f32) -> Result<(), String> {
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_TEMPO, (val - 1.0) * 100.0)
    }
    pub fn set_volume(&mut self, val: f32) -> Result<(), String> {
        Bass::channel_set_attribute(self.handle, 2, val)
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        unsafe { BASS_Free() };
    }
}