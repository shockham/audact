extern crate cpal;
extern crate futures;

use std::thread;
use std::time::Duration;

pub mod system;
use system::{ Audact, Wave };

fn main() {
    let mut audact = Audact::new();

    audact.voice_channel(100.0, Wave::Sine).unwrap();
    audact.voice_channel(200.0, Wave::Square).unwrap();

    let sequences = vec![
        [0,4,8,12],
        [2,6,10,14],
    ];

    let bpm_duration = Duration::from_millis(500); // 120 bpm

    let mut tmp_voice_channels = audact.voice_channels;

    thread::spawn(move || {
        loop {
            // simple 16-step sequencer
            for step in 0 .. 16 {
                for i in 0 .. tmp_voice_channels.len() {
                    if let Ok(_) = sequences[i].binary_search(&step) {
                        tmp_voice_channels[i].play();
                    } else {
                        tmp_voice_channels[i].pause();
                    }
                }
                thread::sleep(bpm_duration);
            }
        }
    });

    (*audact.event_loop).run();
}

