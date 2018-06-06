extern crate audact;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};

fn main() {
    let mut pattern_1 = Audact::new(16, 100, 4f32);
    let n_1 = std_note_freq(0);
    pattern_1.channel(
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![
            n_1, n_1, 0f32, 0f32, n_1, n_1, 0f32, 0f32, n_1, n_1, 0f32, 0f32, n_1, 0f32, 0f32, 0f32,
        ],
    );

    let mut pattern_2 = Audact::new(16, 100, 4f32);
    let n_2 = std_note_freq(4);
    pattern_2.channel(
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![
            n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2, n_2,
        ],
    );

    // play the patterns one after another
    pattern_1.start(1);
    pattern_2.start(1);
}
