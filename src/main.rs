mod audio;
use audio::MusicPlayer;
fn main() -> Result<(), String> {
    println!("{:#x}", unsafe { audio::BASS_GetVersion() });
    println!("{:#x}", unsafe { audio::BASS_FX_GetVersion() });

    let mut mp = MusicPlayer::init()?;
    mp.load("assets/sounds/test.mp3")?;
    mp.set_speed(1.2)?;
    mp.set_volume(0.1)?;
    mp.seek(0.0)?;
    mp.play()?;
    loop {
        println!("{}",mp.pos()?);
    }
}

// find ./src | entr -cs 'cargo run'
