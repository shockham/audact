extern crate audact;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    let c = std_note_freq(0);

    // single test tone
    audact.channel(
        Wave::Sine,
        0.7f32,
        Processing::default(),
        vec![c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c],
    );

    audact.start(1);
}
