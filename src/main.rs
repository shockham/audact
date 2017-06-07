extern crate cpal;
extern crate futures;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use cpal::{ UnknownTypeBuffer, Endpoint, EventLoop, Voice };

use std::sync::Arc;
use std::thread;
use std::time::Duration;

enum Wave {
    Sine,
    Square,
}

struct Audact {
    endpoint: Endpoint,
    event_loop: Arc<EventLoop>,
    voice_channels: Vec<Voice>,
}

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

fn sine_wave(t:f32) -> f32 {
    t.sin()
}

fn square_wave(t:f32) -> f32 {
    t.sin().round()
}

impl Audact {
    fn new() -> Audact {
        let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
        let event_loop = Arc::new(EventLoop::new());

        Audact {
            endpoint: endpoint,
            event_loop: event_loop,
            voice_channels: Vec::new(),
        }
    }

    fn voice_channel(&mut self, freq: f32, wave: Wave) -> Result<bool, bool> {
        let format = self.endpoint.get_supported_formats_list()
            .unwrap().next().expect("Failed to get endpoint format");
        let (voice, stream) = cpal::Voice::new(&self.endpoint, &format,
                                                   &self.event_loop).expect("Failed to create a voice");

        let wave = match wave {
            Wave::Sine => sine_wave,
            Wave::Square => square_wave,
        };

        let samples_rate = format.samples_rate.0 as f32;
        let mut data_source = (0u64..).map(move |t| t as f32 * freq * 2.0 * 3.141592 / samples_rate)
            .map(wave);

        let task = stream.for_each(move |buffer| -> Result<_, ()> {
            match buffer {
                UnknownTypeBuffer::F32(mut buffer) => {
                    for (out, value) in buffer.iter_mut().zip(&mut data_source) {
                        *out = value;
                    }
                },
                _ => (),
            };

            Ok(())
        });

        {
            struct AudactExecutor;
            impl Executor for AudactExecutor {
                fn execute(&self, r: Run) {
                    r.run();
                }
            }
            task::spawn(task).execute(Arc::new(AudactExecutor));
        }

        self.voice_channels.push(voice);

        Ok(true)
    }
}
