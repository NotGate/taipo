mod audio;
use audio::MusicPlayer;
fn main() -> Result<(), &'static str> {
    println!("{:#x}", unsafe { audio::BASS_GetVersion() });
    println!("{:#x}", unsafe { audio::BASS_FX_GetVersion() });

    let mut mp = MusicPlayer::init();
    mp.load("test.mp3")?;
    let mut p1 = mp.pos()?;
    loop {
        println!("{}",mp.pos()?-p1);
        p1 = mp.pos()?;
    }
}

// find ./src | entr -cs 'cargo run'
