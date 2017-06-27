extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes;

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    //lead
    audact.voice_channel(notes::C3, Wave::Saw, (0f32, 1f32), vec![0,4,8,12]).unwrap();
    audact.voice_channel(notes::D3, Wave::Saw, (0f32, 1f32), vec![1,5,9,13]).unwrap();
    audact.voice_channel(notes::F3, Wave::Saw, (0f32, 1f32), vec![2,3,6,7,10,11,14,15]).unwrap();

    //pad
    audact.voice_channel(notes::C3, Wave::Square, (0f32, 1f32), vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.voice_channel(notes::D3, Wave::Square, (0f32, 1f32), vec![8,9,10,11]).unwrap();
    audact.voice_channel(notes::F3, Wave::Square, (0f32, 1f32), vec![12,13,14,15]).unwrap();

    //bass
    audact.voice_channel(notes::C2, Wave::Sine, (0f32, 1f32), vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.voice_channel(notes::F2, Wave::Sine, (0f32, 1f32), vec![8,9,10,11,12,13,14,15]).unwrap();

    // percussion
    audact.voice_channel(notes::C4, Wave::Noise, (0f32, 1f32), vec![2,6,10,14]).unwrap();
    audact.voice_channel(notes::C4, Wave::Noise, (0f32, 0.35f32), vec![0,4,8,12]).unwrap();

    Audact::start(audact);
}

