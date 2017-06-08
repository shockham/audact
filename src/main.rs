/*!
Simple synth and sequencing lib
*/

#![deny(missing_docs)]

extern crate cpal;
extern crate futures;


/// Module for the main audact system
pub mod system;
use system::{ Audact, Wave };

fn main() {
    let mut audact = Audact::new(16, 120);

    audact.voice_channel(100.0, Wave::Sine, vec![0,4,8,12]).unwrap();
    audact.voice_channel(200.0, Wave::Square, vec![2,6,10,14]).unwrap();

    Audact::start(audact);
}

