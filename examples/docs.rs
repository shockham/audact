extern crate audact;

use audact::notes::std_note_freq;
use audact::system::{Audact, Processing, Wave};

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    let default_processing = Processing::default();
    let n_1 = std_note_freq(0);
    let n_2 = std_note_freq(2);

    audact.channel(
        Wave::Sine,
        1f32,
        default_processing,
        vec![
            n_1, 0f32, 0f32, 0f32,
            n_1, 0f32, 0f32, 0f32,
            n_1, 0f32, 0f32, 0f32,
            n_1, 0f32, 0f32, 0f32,
        ],
    );
    audact.channel(Wave::Square, 1f32, default_processing,
        vec![
            0f32, 0f32, n_2, 0f32,
            0f32, 0f32, n_2, 0f32,
            0f32, 0f32, n_2, 0f32,
            0f32, 0f32, n_2, 0f32,
        ],
    );

    audact.start(1);
}
