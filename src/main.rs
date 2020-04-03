mod audio;
use audio::MusicPlayer;
fn main() -> Result<(),&'static str> {
    let mut mp = MusicPlayer::init();
    println!("{}",unsafe { audio::BASS_FX_GetVersion() });



    mp.load("/home/notgate/test.opus")?;
    loop {
        println!("{}",mp.pos()?);
    }
}