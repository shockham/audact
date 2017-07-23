# audact
[![crates.io version](https://img.shields.io/crates/v/audact.svg)](https://crates.io/crates/audact)
[![Build status](https://travis-ci.org/shockham/audact.svg?branch=master)](https://travis-ci.org/shockham/audact)
[![Documentation](https://docs.rs/audact/badge.svg)](https://docs.rs/audact)

Simple synth and sequencing lib

Just has simple sine, square, saw and noise waveforms at the moment. Planning on putting in filters.

Usage:

```rust
extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    audact.channel(notes::C3, Wave::Sine, 1f32, (0f32, 1f32), vec![0,4,8,12]).unwrap();
    audact.channel(notes::D3, Wave::Square, 1f32, (0f32, 1f32), vec![2,6,10,14]).unwrap();

    Audact::start(audact);
}
```
