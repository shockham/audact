#![feature(iterator_flatten)]

extern crate audact;
extern crate rand;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};
use std::iter;
use rand::Rng;


fn main() {

    let seq_len = 16;

    let mut audact = Audact::new(seq_len, 100, 4f32);

    let mut rng = rand::thread_rng();

    let seq:Vec<f32> = (0 .. (seq_len as f32 / 4f32) as usize)
        .map(|_| iter::repeat(std_note_freq(rng.gen_range(-12, 12))).take(4))
        .flatten()
        .collect();

    // single test tone
    audact.channel(
        Wave::Sine,
        0.7f32,
        Processing::default(),
        seq
    );

    audact.start(1);
}
