extern crate audact;

use audact::system::{Audact, Wave, ProcessingBuilder};
use audact::notes::std_note_freq;

use std::time::Duration;

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    let lead_processing = ProcessingBuilder::default()
        .attack(Duration::from_millis(100u64))
        .build()
        .unwrap();

    let default_processing = ProcessingBuilder::default().build().unwrap();

    //lead
    audact.channel(
        std_note_freq(0),
        Wave::Saw,
        0.1f32,
        lead_processing,
        vec![0, 4, 8, 12],
    );
    audact.channel(
        std_note_freq(-2),
        Wave::Saw,
        0.1f32,
        lead_processing,
        vec![1, 5, 9, 13],
    );
    audact.channel(
        std_note_freq(-4),
        Wave::Saw,
        0.1f32,
        lead_processing,
        vec![2, 3, 6, 7, 10, 11, 14, 15],
    );

    //pad
    audact.channel(
        std_note_freq(0),
        Wave::Square,
        0.1f32,
        default_processing,
        vec![0, 1, 2, 3, 4, 5, 6, 7],
    );
    audact.channel(
        std_note_freq(-2),
        Wave::Square,
        0.1f32,
        default_processing,
        vec![8, 9, 10, 11],
    );
    audact.channel(
        std_note_freq(-4),
        Wave::Square,
        0.1f32,
        default_processing,
        vec![12, 13, 14, 15],
    );

    //bass
    audact.channel(
        std_note_freq(-12),
        Wave::Sine,
        0.1f32,
        default_processing,
        vec![0, 1, 2, 3, 4, 5, 6, 7],
    );
    audact.channel(
        std_note_freq(-13),
        Wave::Sine,
        0.1f32,
        default_processing,
        vec![8, 9, 10, 11, 12, 13, 14, 15],
    );

    // percussion
    audact.channel(
        std_note_freq(0),
        Wave::Noise,
        0.2f32,
        default_processing,
        vec![2, 6, 10, 14],
    );
    audact.channel(
        std_note_freq(0),
        Wave::Noise,
        0.2f32,
        default_processing,
        vec![0, 4, 8, 12],
    );

    Audact::start(audact, 1);
}
