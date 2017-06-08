use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use cpal;
use cpal::{ UnknownTypeBuffer, Endpoint, EventLoop, Voice };

use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Enum of available waveforms
pub enum Wave {
    /// Sine waveform
    Sine,
    /// Square waveform
    Square,
}

/// Struct for the main audact system
pub struct Audact {
    /// The endpoint that audact will play through
    endpoint: Endpoint,
    /// The cpal event loop
    pub event_loop: Arc<EventLoop>,
    /// Vec of voice channels that audact will play
    pub voice_channels: Vec<(Voice, Vec<i32>)>,
}

/// implementation for the audact struct
impl Audact {
    /// Creates a new instance of audact
    pub fn new() -> Audact {
        let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
        let event_loop = Arc::new(EventLoop::new());

        Audact {
            endpoint: endpoint,
            event_loop: event_loop,
            voice_channels: Vec::new(),
        }
    }

    /// Generates a sine wave from samples
    fn sine_wave(t:f32) -> f32 {
        t.sin()
    }

    /// Generates a square wave from samples
    fn square_wave(t:f32) -> f32 {
        t.sin().round()
    }

    /// Add a voice channel to audact for synth playback
    pub fn voice_channel(&mut self, freq: f32, wave: Wave, seq: Vec<i32>) -> Result<bool, bool> {
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

        self.voice_channels.push((voice, seq));

        Ok(true)
    }

    /// Kick off audact to start
    pub fn start(audact:Audact) {
        let bpm_duration = Duration::from_millis(500); // 120 bpm

        let mut tmp_voice_channels = audact.voice_channels;

        thread::spawn(move || {
            loop {
                // simple 16-step sequencer
                for step in 0 .. 16 {
                    for i in 0 .. tmp_voice_channels.len() {
                        if let Ok(_) = tmp_voice_channels[i].1.binary_search(&step) {
                            tmp_voice_channels[i].0.play();
                        } else {
                            tmp_voice_channels[i].0.pause();
                        }
                    }
                    thread::sleep(bpm_duration);
                }
            }
        });

        (*audact.event_loop).run();
    }
}
