extern crate audact;
extern crate rand;

use audact::notes::std_note_freq;
use audact::system::{Audact, ProcessingBuilder, Wave};
use std::iter;
use rand::Rng;
use std::time::Duration;


fn main() {
    let seq_len = 16;

    let mut audact = Audact::new(seq_len, 100, 4f32);

    let mut rng = rand::thread_rng();

    let seq:Vec<f32> = (0 .. (seq_len as f32 / 4f32) as usize)
        .map(|_| iter::repeat(std_note_freq(rng.gen_range(-12, 12))).take(4))
        .flat_map(|x| x)
        .collect();

    let processing = ProcessingBuilder::default()
        .attack(Duration::from_millis(300u64))
        .reverb((Duration::from_millis(200), 0.8f32))
        .build()
        .unwrap();

    // single test tone
    audact.channel(
        Wave::Sine,
        0.3f32,
        processing,
        seq
    );

    audact.start(4);
}
