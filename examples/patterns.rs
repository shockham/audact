extern crate audact;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};

fn main() {
    let mut pattern_1 = Audact::new(16, 100, 4f32);
    pattern_1.channel(
        std_note_freq(0),
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![0, 1, 4, 5, 8, 9, 12],
    );

    let mut pattern_2 = Audact::new(16, 100, 4f32);
    pattern_2.channel(
        std_note_freq(4),
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );

    // play the patterns one after another
    pattern_1.start(1);
    pattern_2.start(1);
}
