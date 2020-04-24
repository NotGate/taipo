#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fs::File, io::Read, ptr};
include!("bass/wrapper.rs");
include!("bass/bindings.rs");

pub struct MusicPlayer {
    handle: u32,
    volume: f32,
    buffer: Vec<u8>,
}

impl MusicPlayer {
    pub fn init() -> Result<MusicPlayer, String> {
        // TODO: fine-tune configs
        Bass::set_config(BASS_CONFIG_BUFFER, 5000)?; // 5000 ms instead of 500 ms (lower is choppier)
        Bass::set_config(BASS_CONFIG_DEV_NONSTOP, 1)?; // more consistent playback latency

        Bass::init(44100, 0)?;
        Ok(MusicPlayer {
            handle: 0,
            volume: 0.0,
            buffer: vec![],
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
            Bass::stream_create_file(
                &self.buffer,
                // TODO: fine-tune flags
                BASS_STREAM_DECODE
                    | if Bass::get_config(BASS_CONFIG_FLOAT)? != 0 {
                        BASS_SAMPLE_FLOAT // better quality
                    } else {
                        0
                    }
                    | BASS_STREAM_PRESCAN // better length reading in MP3 and chained OGG (takes longer to create stream)
                    | BASS_UNICODE,
            )?,
            BASS_FX_FREESOURCE | BASS_MUSIC_LOOP,
        )?;
        // TODO: fine-tune attributes
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_TEMPO_OPTION_USE_QUICKALGO, 1.0)?;
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_TEMPO_OPTION_SEQUENCE_MS, 30.0)?;
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_TEMPO_OPTION_OVERLAP_MS, 4.0)?;
        Ok(())
    }
    pub fn seek(&self, pos: f64) -> Result<(), String> {
        Bass::channel_set_position(
            self.handle,
            Bass::channel_seconds2bytes(self.handle, num::clamp(pos, 0.0, self.len()? - 1.0))?,
        )
    }
    pub fn pos(&self) -> Result<f64, String> {
        Bass::channel_bytes2seconds(self.handle, Bass::channel_get_position(self.handle)?)
    }
    pub fn len(&self) -> Result<f64, String> {
        Bass::channel_bytes2seconds(self.handle, Bass::channel_get_length(self.handle)?)
    }
    pub fn play(&self) -> Result<(), String> {
        Bass::channel_play(self.handle, 0)
    }
    pub fn pause(&self) -> Result<(), String> {
        Bass::channel_pause(self.handle)
    }
    pub fn mute(&mut self) -> Result<(), String> {
        self.volume = self.get_volume()?;
        self.set_volume(0.0)
    }
    pub fn unmute(&self) -> Result<(), String> {
        self.set_volume(self.volume)
    }
    pub fn set_speed(&self, val: f32) -> Result<(), String> {
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_TEMPO, (num::clamp(val, 0.5, 2.0) - 1.0) * 100.0)
    }
    pub fn set_volume(&self, val: f32) -> Result<(), String> {
        Bass::channel_set_attribute(self.handle, BASS_ATTRIB_VOL, num::clamp(val, 0.0, 1.0))
    }
    pub fn get_speed(&self) -> Result<f32, String> {
        Ok(1.0 + (Bass::channel_get_attribute(self.handle, BASS_ATTRIB_TEMPO)? / 100.0))
    }
    pub fn get_volume(&self) -> Result<f32, String> {
        Bass::channel_get_attribute(self.handle, BASS_ATTRIB_VOL)
    }
    pub fn is_playing(&self) -> Result<bool, String> {
        Bass::channel_is_active(self.handle).map(|d: DWORD| if d == 1 { true } else { false })
    }
    pub fn get_latency(&self) -> Result<u32, String> {
        Bass::get_info().map(|s: BASS_INFO| s.latency)
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        unsafe { BASS_Free() };
    }
}
