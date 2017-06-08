use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use cpal;
use cpal::{ UnknownTypeBuffer, Endpoint, EventLoop, Voice };

use std::sync::Arc;


pub enum Wave {
    Sine,
    Square,
}

pub struct Audact {
    endpoint: Endpoint,
    pub event_loop: Arc<EventLoop>,
    pub voice_channels: Vec<Voice>,
}

impl Audact {
    pub fn new() -> Audact {
        let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
        let event_loop = Arc::new(EventLoop::new());

        Audact {
            endpoint: endpoint,
            event_loop: event_loop,
            voice_channels: Vec::new(),
        }
    }

    fn sine_wave(t:f32) -> f32 {
        t.sin()
    }

    fn square_wave(t:f32) -> f32 {
        t.sin().round()
    }

    pub fn voice_channel(&mut self, freq: f32, wave: Wave) -> Result<bool, bool> {
        let format = self.endpoint.get_supported_formats_list()
            .unwrap().next().expect("Failed to get endpoint format");
        let (voice, stream) = cpal::Voice::new(&self.endpoint, &format,
                                                   &self.event_loop).expect("Failed to create a voice");

        let wave = match wave {
            Wave::Sine => Audact::sine_wave,
            Wave::Square => Audact::square_wave,
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
