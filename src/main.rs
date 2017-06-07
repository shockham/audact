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
}

fn main() {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let event_loop = Arc::new(EventLoop::new());

    let audact = Audact {
        endpoint: endpoint,
        event_loop: event_loop,
    };

    let mut voice_steps = vec![
        (voice_channel(&audact.endpoint,  &audact.event_loop, 100.0, Wave::Sine), [0,4,8,12]),
        (voice_channel(&audact.endpoint,  &audact.event_loop, 200.0, Wave::Square), [2,6,10,14]),
    ];

    let bpm_duration = Duration::from_millis(500); // 120 bpm

    thread::spawn(move || {
        loop {
            // simple 16-step sequencer
            for step in 0 .. 16 {
                for i in 0 .. voice_steps.len() {
                    if let Ok(_) = voice_steps[i].1.binary_search(&step) {
                        voice_steps[i].0.play();
                    } else {
                        voice_steps[i].0.pause();
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

fn voice_channel(endpoint: &Endpoint, event_loop: &Arc<EventLoop>, freq: f32, wave: Wave) -> Voice {
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");
    let (voice, stream) = cpal::Voice::new(&endpoint, &format,
                                               &event_loop).expect("Failed to create a voice");

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

    voice
}
