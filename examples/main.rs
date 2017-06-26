extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    //lead
    audact.voice_channel(notes::C4, Wave::Saw, vec![0,4,8,12]).unwrap();
    audact.voice_channel(notes::D4, Wave::Saw, vec![1,5,9,13]).unwrap();
    audact.voice_channel(notes::F4, Wave::Saw, vec![2,3,6,7,10,11,14,15]).unwrap();

    //pad
    audact.voice_channel(notes::C3, Wave::Square, vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.voice_channel(notes::D3, Wave::Square, vec![8,9,10,11]).unwrap();
    audact.voice_channel(notes::F3, Wave::Square, vec![12,13,14,15]).unwrap();

    //bass
    audact.voice_channel(notes::C3, Wave::Sine, vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.voice_channel(notes::F3, Wave::Sine, vec![8,9,10,11,12,13,14,15]).unwrap();

    // percussion
    audact.voice_channel(150.0, Wave::Noise, vec![2,6,10,14]).unwrap();

    Audact::start(audact);
}

