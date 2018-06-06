extern crate audact;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    // single test tone
    audact.channel(
        std_note_freq(0),
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );

    audact.start(1);
}
