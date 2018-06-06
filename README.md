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

use audact::system::{ Audact, Wave, Processing };
use audact::notes::std_note_freq;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    let default_processing = Processing::default();
    let n_1 = std_note_freq(0);
    let n_2 = std_note_freq(2);

    audact.channel(Wave::Sine, 1f32, default_processing,
        vec![n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32]);
    audact.channel(Wave::Square, 1f32, default_processing,
        vec![2,6,10,14]);

    audact.start(1);
}
```
