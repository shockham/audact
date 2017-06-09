extern crate audact;

use audact::system::{ Audact, Wave };

fn main() {
    let mut audact = Audact::new(16, 120);

    //lead
    audact.voice_channel(100.0, Wave::Square, vec![0,4,8,12]).unwrap();
    audact.voice_channel(150.0, Wave::Square, vec![1,5,9,13]).unwrap();
    audact.voice_channel(200.0, Wave::Square, vec![2,3,6,7,10,11,14,15]).unwrap();

    //bass
    audact.voice_channel(50.0, Wave::Sine, vec![0,1,2,3,4,5,6,7]).unwrap();
    audact.voice_channel(75.0, Wave::Sine, vec![8,9,10,11,12,13,14,15]).unwrap();

    // percussion
    audact.voice_channel(150.0, Wave::Noise, vec![0,4,8,12]).unwrap();

    Audact::start(audact);
}

