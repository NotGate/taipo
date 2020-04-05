macro_rules! call {
    ($name:expr,$func:expr) => {{
        unsafe { $func };
        match Bass::get_error() {
            Some(error) => Err(format!("{} failed: {}", $name, error)),
            None => Ok(()),
        }
    }};
}

macro_rules! callv {
    ($name:expr,$func:expr) => {{
        let v = unsafe { $func };
        match Bass::get_error() {
            Some(error) => Err(format!("{} failed: {}", $name, error)),
            None => Ok(v),
        }
    }};
}

struct Bass {}
impl Bass {
    pub fn init(freq: DWORD, flags: DWORD) -> Result<(), String> {
        call!("init", BASS_Init(-1, freq, flags, ptr::null_mut(), ptr::null_mut()))
    }
    pub fn stream_free(handle: HSTREAM) -> Result<(), String> {
        call!("stream_free", BASS_StreamFree(handle))
    }
    pub fn channel_seconds2bytes(handle: DWORD, pos: f64) -> Result<QWORD, String> {
        callv!("channel_seconds2bytes", BASS_ChannelSeconds2Bytes(handle, pos))
    }
    pub fn channel_bytes2seconds(handle: DWORD, pos: QWORD) -> Result<f64, String> {
        callv!("channel_bytes2seconds", BASS_ChannelBytes2Seconds(handle, pos))
    }
    pub fn channel_set_position(handle: DWORD, pos: QWORD) -> Result<(), String> {
        call!("channel_set_position", BASS_ChannelSetPosition(handle, pos, BASS_POS_BYTE))
    }
    pub fn channel_get_position(handle: DWORD) -> Result<QWORD, String> {
        callv!("channel_get_position", BASS_ChannelGetPosition(handle, BASS_POS_BYTE))
    }
    pub fn channel_get_length(handle: DWORD) -> Result<QWORD, String> {
        callv!("channel_get_length", BASS_ChannelGetLength(handle, BASS_POS_BYTE))
    }
    pub fn fx_tempo_create(chan: DWORD, flags: DWORD) -> Result<HSTREAM, String> {
        callv!("fx_tempo_create", BASS_FX_TempoCreate(chan, flags))
    }
    pub fn channel_play(handle: DWORD, restart: BOOL) -> Result<(), String> {
        call!("channel_play", BASS_ChannelPlay(handle, restart))
    }
    pub fn channel_pause(handle: DWORD) -> Result<(), String> {
        call!("channel_pause", BASS_ChannelPause(handle))
    }
    // http://www.un4seen.com/doc/#bass/BASS_ChannelSetAttribute.html
    pub fn channel_set_attribute(handle: DWORD, attrib: DWORD, val: f32) -> Result<(), String> {
        call!("channel_set_attribute", BASS_ChannelSetAttribute(handle, attrib, val))
    }
    pub fn channel_get_attribute(handle: DWORD, attrib: DWORD) -> Result<f32, String> {
        let mut v = 0.0;
        call!("channel_get_attribute", BASS_ChannelGetAttribute(handle, attrib, &mut v))?;
        Ok(v)
    }
    pub fn channel_is_active(handle: DWORD) -> Result<DWORD,String> {
        callv!("channel_is_active",BASS_ChannelIsActive(handle))
    }
    pub fn get_config(option: DWORD) -> Result<DWORD,String> {
        callv!("get_config", BASS_GetConfig(option))
    }
    // http://www.un4seen.com/doc/#bass/BASS_SetConfig.html
    pub fn set_config(option: DWORD, value: DWORD) -> Result<(), String> {
        call!("set_config", BASS_SetConfig(option, value))
    }
    // http://www.un4seen.com/doc/#bass/BASS_INFO.html
    pub fn get_info() -> Result<BASS_INFO, String> {
        let mut info: BASS_INFO = unsafe { core::mem::zeroed() };
        call!("get_info",BASS_GetInfo(&mut info))?;
        Ok(info)
    }
    pub fn stream_create_file(buffer: &[u8], flags: DWORD) -> Result<HSTREAM, String> {
        callv!(
            "stream_create_file",
            BASS_StreamCreateFile(
                1,
                buffer.as_ptr() as *const std::ffi::c_void,
                0,
                buffer.len() as u64,
                flags,
            )
        )
    }
    pub fn get_error() -> Option<String> {
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
    
}