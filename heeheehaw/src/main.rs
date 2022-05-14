use soloud::*;
use std::{thread, time::Duration};
fn main() {
    loop{
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("funny.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    thread::sleep(Duration::from_millis(4000));
    }

}
