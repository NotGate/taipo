#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fs::File, io::Read, ptr};
include!("bass_bindings.rs");

pub struct MusicPlayer {
    handle: u32,
    buffer: Vec<u8>,
    pub playing: bool,
    pub looping: bool,
    pub speed: f32,
    pub volume: f32,
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        unsafe { BASS_Free() };
    }
}

macro_rules! check {
    ($name:expr,$value:expr) => {
        match get_error() {
            Some(error) => Err(format!("{} failed: {}", $name, error)),
            None => Ok($value),
        };
    };
}

impl MusicPlayer {
    pub fn init() -> Result<MusicPlayer, String> {
        unsafe { BASS_Init(-1, 44100, 0, ptr::null_mut(), ptr::null_mut()) };
        return check!(
            "BASS_Init",
            MusicPlayer {
                handle: 0,
                buffer: vec![],
                playing: false,
                looping: false,
                speed: 1.0,
                volume: 0.5,
            }
        )
    }
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        if self.handle != 0 {
            unsafe { BASS_StreamFree(self.handle) };
            return check!("BASS_StreamFree", ());
        }
        let mut f = File::open(path).expect("Could not open audio file");
        self.buffer = Vec::new();
        f.read_to_end(&mut self.buffer).expect("Could not read audio file");
        let ptr = self.buffer.as_ptr();
        let chan = unsafe { BASS_StreamCreateFile(
            1,
            ptr as *const std::ffi::c_void,
            0,
            self.buffer.len() as u64,
            BASS_STREAM_DECODE,
        )};
        if let Err(e) = check!("BASS_StreamCreateFile", ()) {
            return Err(e)
        }
        self.handle = unsafe { BASS_FX_TempoCreate(chan, BASS_FX_FREESOURCE | BASS_MUSIC_LOOP) };
        if let Err(e) = check!("BASS_FX_TempoCreate", ()) {
            return Err(e)
        }
        Ok(())
    }
    pub fn seek(&mut self, val: f64) -> Result<(), String> {
        let bytes = unsafe { BASS_ChannelSeconds2Bytes(self.handle, val) };
        if let Err(e) = check!("BASS_ChannelSeconds2Bytes", ()) {
            return Err(e)
        }
        unsafe { BASS_ChannelSetPosition(self.handle, bytes, 0) };
        return check!("BASS_ChannelSetPosition", ())
    }
    pub fn pos(&self) -> Result<f64, String> {
        let position = unsafe { BASS_ChannelGetPosition(self.handle, 0) };
        if let Err(e) = check!("BASS_ChannelGetPosition", ()) {
            return Err(e)
        }
        let seconds = unsafe { BASS_ChannelBytes2Seconds(self.handle, position) };
        return check!("BASS_ChannelGetPosition", seconds)
    }
    pub fn play(&mut self) -> Result<(), String> {
        unsafe { BASS_ChannelPlay(self.handle, 0) };
        return check!("BASS_ChannelPlay", ())
    }
    pub fn pause(&mut self) -> Result<(), String> {
        unsafe { BASS_ChannelPause(self.handle) };
        return check!("BASS_ChannelPause", ())
    }
    pub fn set_speed(&mut self, val: f32) -> Result<(), String> {
        set_attribute(self.handle, BASS_ATTRIB_TEMPO, (val - 1.0) * 100.0)
    }
    pub fn set_volume(&mut self, val: f32) -> Result<(), String> {
        set_attribute(self.handle, 2, val)
    }
}

// Wrapper
fn set_attribute(handle: u32, attrib: u32, val: f32) -> Result<(), String> {
    unsafe { BASS_ChannelSetAttribute(handle, attrib, val) };
    return check!("BASS_ChannelSetAttribute", ())
}
fn fail(s: String) -> Result<(), String> {
    if let Some(e) = get_error() {
        Err(format!("{} failed: {}", s, e))
    } else {
        Ok(())
    }
}
fn get_error() -> Option<String> {
    match unsafe { BASS_ErrorGetCode() } {
        0 => None,
        1 => Some("BASS_ERROR_MEM".into()),
        2 => Some("BASS_ERROR_FILEOPEN".into()),
        3 => Some("BASS_ERROR_DRIVER".into()),
        4 => Some("BASS_ERROR_BUFLOST".into()),
        5 => Some("BASS_ERROR_HANDLE".into()),
        6 => Some("BASS_ERROR_FORMAT".into()),
        7 => Some("BASS_ERROR_POSITION".into()),
        8 => Some("BASS_ERROR_INIT".into()),
        9 => Some("BASS_ERROR_START".into()),
        10 => Some("BASS_ERROR_SSL".into()),
        14 => Some("BASS_ERROR_ALREADY".into()),
        17 => Some("BASS_ERROR_NOTAUDIO".into()),
        18 => Some("BASS_ERROR_NOCHAN".into()),
        19 => Some("BASS_ERROR_ILLTYPE".into()),
        20 => Some("BASS_ERROR_ILLPARAM".into()),
        21 => Some("BASS_ERROR_NO3D".into()),
        22 => Some("BASS_ERROR_NOEAX".into()),
        23 => Some("BASS_ERROR_DEVICE".into()),
        24 => Some("BASS_ERROR_NOPLAY".into()),
        25 => Some("BASS_ERROR_FREQ".into()),
        27 => Some("BASS_ERROR_NOTFILE".into()),
        29 => Some("BASS_ERROR_NOHW".into()),
        31 => Some("BASS_ERROR_EMPTY".into()),
        32 => Some("BASS_ERROR_NONET".into()),
        33 => Some("BASS_ERROR_CREATE".into()),
        34 => Some("BASS_ERROR_NOFX".into()),
        37 => Some("BASS_ERROR_NOTAVAIL".into()),
        38 => Some("BASS_ERROR_DECODE".into()),
        39 => Some("BASS_ERROR_DX".into()),
        40 => Some("BASS_ERROR_TIMEOUT".into()),
        41 => Some("BASS_ERROR_FILEFORM".into()),
        42 => Some("BASS_ERROR_SPEAKER".into()),
        43 => Some("BASS_ERROR_VERSION".into()),
        44 => Some("BASS_ERROR_CODEC".into()),
        45 => Some("BASS_ERROR_ENDED".into()),
        46 => Some("BASS_ERROR_BUSY".into()),
        47 => Some("BASS_ERROR_UNSTREAMABLE".into()),
        -1 => Some("BASS_ERROR_UNKNOWN".into()),
        _ => Some("Undefined Error!".into()),
    }
}
