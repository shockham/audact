/*!
Simple synth and sequencing lib.

Just has simple sine, square, saw and noise waveforms at the moment. Planning on putting in filters.

Usage:

```no_run
extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes::std_note_freq;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    audact.channel(std_note_freq(0), Wave::Sine, 1f32, (0f32, 1f32), vec![0,4,8,12]).unwrap();
    audact.channel(std_note_freq(2), Wave::Square, 1f32, (0f32, 1f32), vec![2,6,10,14]).unwrap();

    Audact::start(audact, 1);
}
```

*/

#![deny(missing_docs)]

extern crate cpal;
extern crate futures;
extern crate rand;

/// Module for the main audact system
pub mod system;
/// Module containing note frequencies
pub mod notes;
