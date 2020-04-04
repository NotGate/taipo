#![allow(dead_code)]

use libc;
use std::{fs::File, io::Read};

/*
TODO:
enums and consts
cleaner load
test with: opus, mp3, wav, etc
youtube-dl https://www.youtube.com/watch?v=sP7lp8UD1Xw -x --audio-format "mp3" -o test.mp3
*/

// Raw FFI
const BASS_FX_FREESOURCE: u32 = 0x10000;
const BASS_SAMPLE_LOOP: u32 = 4;
const BASS_ATTRIB_TEMPO: u32 = 0x10000;
const BASS_STREAM_DECODE: u32 = 0x200000;

#[link(name = "bass")]
extern "C" {
    pub fn BASS_GetVersion() -> i32;
    pub fn BASS_ErrorGetCode() -> i32;
    pub fn BASS_Init(
        device: i32,
        freq: u32,
        flags: u32,
        win: libc::uintptr_t,
        dsguid: libc::uintptr_t,
    ) -> i32;
    pub fn BASS_Free() -> i32;
    pub fn BASS_StreamCreateFile(
        mem: i32,
        file: *const u8,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> u32;
    pub fn BASS_StreamFree(handle: u32) -> i32;
    pub fn BASS_ChannelPlay(handle: u32, restart: i32) -> i32;
    pub fn BASS_ChannelPause(handle: u32) -> i32;
    pub fn BASS_ChannelGetPosition(handle: u32, mode: u32) -> u64;
    pub fn BASS_ChannelSetPosition(handle: u32, pos: u64, mode: u32) -> i32;
    pub fn BASS_ChannelSeconds2Bytes(handle: u32, pos: f64) -> u64;
    pub fn BASS_ChannelBytes2Seconds(handle: u32, pos: u64) -> f64;
    pub fn BASS_ChannelSetAttribute(handle: u32, attrib: u32, val: f32) -> i32;
}

#[link(name = "bass_fx")]
extern "C" {
    pub fn BASS_FX_GetVersion() -> i32;
    pub fn BASS_FX_TempoCreate(chan: u32, flags: u32) -> u32;
}

pub struct MusicPlayer {
    handle: u32,
    buffer: Vec<u8>,
    pub playing: bool,
    pub looping: bool,
    pub speed: f32,
    pub volume: f32,
}

impl MusicPlayer {
    pub fn init() -> MusicPlayer {
        let i = unsafe { BASS_Init(-1, 44100, 0, 0, 0) };
        println!("{}", i);
        match get_error() {
            Ok(_) => println!("BASS_Init successful"),
            Err(e) => println!("BASS_Init failed: {}", e),
        }
        MusicPlayer {
            handle: 0,
            buffer: vec![],
            playing: false,
            looping: false,
            speed: 1.0,
            volume: 0.5,
        }
    }
    pub fn load(&mut self, path: &str) -> Result<(), &'static str> {
        println!("1");
        if self.handle != 0 {
            free_stream(self.handle)?;
        }
        println!("2");
        match File::open(path) {
            Ok(mut f) => {
                println!("3");
                self.buffer = Vec::new();
                if let Err(_) = f.read_to_end(&mut self.buffer) {
                    return Err("Could not read audio file");
                }
                println!("4");
                // let chan = create_stream(&self.buffer, BASS_STREAM_DECODE)?;
                // let chan = unsafe { BASS_StreamCreateFile(1, self.buffer.as_ptr(), 0, self.buffer.len() as u64, BASS_STREAM_DECODE) };
                // println!("chan: {}",chan);
                // self.handle = create_tempo(chan, BASS_FX_FREESOURCE | BASS_SAMPLE_LOOP)?;
                // println!("{:?}",self.buffer);
                self.handle = unsafe {
                    let ptr = self.buffer.as_ptr();
                    let chan = BASS_StreamCreateFile(1, ptr, 0, self.buffer.len() as u64, 0x200000); //BASS_STREAM_DECODE
                    match get_error() {
                        Ok(_) => println!("BASS_Init successful"),
                        Err(e) => println!("BASS_StreamCreateFile failed: {}", e),
                    }
                    println!("chan: {}", chan);
                    BASS_FX_TempoCreate(chan, 0x10000 | 4) //BASS_FX_FREESOURCE|BASS_SAMPLE_LOOP
                };
                match get_error() {
                    Ok(_) => println!("BASS_Init successful"),
                    Err(e) => println!("BASS_FX_TempoCreate failed: {}", e),
                }
                println!("handle: {}", self.handle);
                println!("5");
                self.set_speed(self.speed)?;
                println!("6");
                self.set_volume(self.volume)?;
                println!("7");
                self.seek(0.0)?;
                println!("8");
                self.play()
            }
            Err(_) => Err("Could not open audio file"),
        }
    }
    pub fn seek(&mut self, val: f64) -> Result<(), &'static str> {
        unsafe {
            BASS_ChannelSetPosition(self.handle, BASS_ChannelSeconds2Bytes(self.handle, val), 0)
        };
        get_error()
    }
    pub fn pos(&self) -> Result<f64, &'static str> {
        let pos = unsafe {
            BASS_ChannelBytes2Seconds(self.handle, BASS_ChannelGetPosition(self.handle, 0))
        };
        match get_error() {
            Ok(_) => Ok(pos),
            Err(e) => Err(e),
        }
    }
    pub fn play(&mut self) -> Result<(), &'static str> {
        println!("{}", self.handle);
        unsafe { BASS_ChannelPlay(self.handle, 0) };
        println!("8");
        get_error()
    }
    pub fn pause(&mut self) -> Result<(), &'static str> {
        unsafe { BASS_ChannelPause(self.handle) };
        get_error()
    }
    pub fn set_speed(&mut self, val: f32) -> Result<(), &'static str> {
        set_attribute(self.handle, BASS_ATTRIB_TEMPO, (val - 1.0) * 100.0)
    }
    pub fn set_volume(&mut self, val: f32) -> Result<(), &'static str> {
        set_attribute(self.handle, 2, val)
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        unsafe { BASS_Free() };
    }
}

// Wrapper
fn set_attribute(handle: u32, attrib: u32, val: f32) -> Result<(), &'static str> {
    if unsafe { BASS_ChannelSetAttribute(handle, attrib, val) } != 0 {
        get_error()
    } else {
        Ok(())
    }
}
fn create_stream(buffer: &[u8], flags: u32) -> Result<u32, &'static str> {
    let chan = unsafe { BASS_StreamCreateFile(1, buffer.as_ptr(), 0, buffer.len() as u64, flags) };
    if chan != 0 {
        match get_error() {
            Ok(_) => Ok(chan),
            Err(e) => Err(e),
        }
    } else {
        Ok(chan)
    }
}
fn create_tempo(chan: u32, flags: u32) -> Result<u32, &'static str> {
    let handle = unsafe { BASS_FX_TempoCreate(chan, flags) };
    if handle != 0 {
        match get_error() {
            Ok(_) => Ok(handle),
            Err(e) => Err(e),
        }
    } else {
        Ok(handle)
    }
}
fn free_stream(handle: u32) -> Result<(), &'static str> {
    if unsafe { BASS_StreamFree(handle) } != 0 {
        return get_error();
    }
    Ok(())
}
fn get_error() -> Result<(), &'static str> {
    match unsafe { BASS_ErrorGetCode() } {
        0 => Ok(()),
        1 => Err("BASS_ERROR_MEM"),
        2 => Err("BASS_ERROR_FILEOPEN"),
        3 => Err("BASS_ERROR_DRIVER"),
        4 => Err("BASS_ERROR_BUFLOST"),
        5 => Err("BASS_ERROR_HANDLE"),
        6 => Err("BASS_ERROR_FORMAT"),
        7 => Err("BASS_ERROR_POSITION"),
        8 => Err("BASS_ERROR_INIT"),
        9 => Err("BASS_ERROR_START"),
        10 => Err("BASS_ERROR_SSL"),
        14 => Err("BASS_ERROR_ALREADY"),
        17 => Err("BASS_ERROR_NOTAUDIO"),
        18 => Err("BASS_ERROR_NOCHAN"),
        19 => Err("BASS_ERROR_ILLTYPE"),
        20 => Err("BASS_ERROR_ILLPARAM"),
        21 => Err("BASS_ERROR_NO3D"),
        22 => Err("BASS_ERROR_NOEAX"),
        23 => Err("BASS_ERROR_DEVICE"),
        24 => Err("BASS_ERROR_NOPLAY"),
        25 => Err("BASS_ERROR_FREQ"),
        27 => Err("BASS_ERROR_NOTFILE"),
        29 => Err("BASS_ERROR_NOHW"),
        31 => Err("BASS_ERROR_EMPTY"),
        32 => Err("BASS_ERROR_NONET"),
        33 => Err("BASS_ERROR_CREATE"),
        34 => Err("BASS_ERROR_NOFX"),
        37 => Err("BASS_ERROR_NOTAVAIL"),
        38 => Err("BASS_ERROR_DECODE"),
        39 => Err("BASS_ERROR_DX"),
        40 => Err("BASS_ERROR_TIMEOUT"),
        41 => Err("BASS_ERROR_FILEFORM"),
        42 => Err("BASS_ERROR_SPEAKER"),
        43 => Err("BASS_ERROR_VERSION"),
        44 => Err("BASS_ERROR_CODEC"),
        45 => Err("BASS_ERROR_ENDED"),
        46 => Err("BASS_ERROR_BUSY"),
        47 => Err("BASS_ERROR_UNSTREAMABLE"),
        -1 => Err("BASS_ERROR_UNKNOWN"),
        _ => Err("Undefined Error!"),
    }
}
