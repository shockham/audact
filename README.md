# audact
[![crates.io version](https://img.shields.io/crates/v/audact.svg)](https://crates.io/crates/audact)
[![Build status](https://travis-ci.org/shockham/audact.svg?branch=master)](https://travis-ci.org/shockham/audact)
[![Documentation](https://docs.rs/audact/badge.svg)](https://docs.rs/audact)

Minimalist synth and sequencing lib

Contains:
- Simple sine, square, saw and noise waveforms.
- Hard-edge cut-off filters.
- Basic sequencing of a single pattern.

Usage:

```rust
extern crate audact;

use audact::system::{ Audact, Wave, ProcessingBuilder };
use audact::notes::std_note_freq;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    let default_processing = ProcessingBuilder::default().build().unwrap();

    audact.channel(std_note_freq(0), Wave::Sine, 1f32, default_processing, vec![0,4,8,12]);
    audact.channel(std_note_freq(2), Wave::Square, 1f32, default_processing, vec![2,6,10,14]);

    Audact::start(audact, 1);
}
```
