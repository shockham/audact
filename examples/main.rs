extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes::std_note_freq;

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    //lead
    audact.channel(std_note_freq(-12), Wave::Saw, 1f32, (0f32, 0.8f32), vec![0,4,8,12]).unwrap();
    audact.channel(std_note_freq(-14), Wave::Saw, 1f32, (0f32, 0.8f32), vec![1,5,9,13]).unwrap();
    audact.channel(std_note_freq(-16), Wave::Saw, 1f32, (0f32, 0.8f32), vec![2,3,6,7,10,11,14,15]).unwrap();

    //pad
    audact.channel(std_note_freq(-12), Wave::Square, 1f32, (0f32, 1f32), vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.channel(std_note_freq(-14), Wave::Square, 1f32, (0f32, 1f32), vec![8,9,10,11]).unwrap();
    audact.channel(std_note_freq(-16), Wave::Square, 1f32, (0f32, 1f32), vec![12,13,14,15]).unwrap();

    //bass
    audact.channel(std_note_freq(-24), Wave::Sine, 1f32, (0f32, 1f32), vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.channel(std_note_freq(-26), Wave::Sine, 1f32, (0f32, 1f32), vec![8,9,10,11,12,13,14,15]).unwrap();

    // percussion
    audact.channel(std_note_freq(0), Wave::Noise, 1f32, (0.65f32, 1f32), vec![2,6,10,14]).unwrap();
    audact.channel(std_note_freq(0), Wave::Noise, 1f32, (0f32, 0.35f32), vec![0,4,8,12]).unwrap();

    Audact::start(audact);
}

