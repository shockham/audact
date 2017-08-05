use rodio;
use rodio::Endpoint;
use rodio::Sink;
use rodio::source;
use rodio::buffer::SamplesBuffer;
use rodio::Source;

use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

use rand::random;


/// Enum of available waveforms
pub enum Wave {
    /// Sine waveform
    Sine,
    /// Square waveform
    Square,
    /// Saw-tooth waveform
    Saw,
    /// White noise waveform
    Noise,
}

/// Struct for the main audact system
pub struct Audact {
    /// The endpoint that audact will play through
    endpoint: Endpoint,
    /// Vec of voice channels that audact will play
    channels: Vec<(Sink, Vec<i32>)>,
    /// The number of steps for the sequencer
    steps: i32,
    /// The duraction that determines the bpm
    bpm_duration: Duration,
}

/// implementation for the audact struct
impl Audact {
    /// Creates a new instance of audact
    pub fn new(steps:i32, bpm:i32, per_bar:f32) -> Audact {
        let endpoint = rodio::get_default_endpoint().unwrap();

        Audact {
            endpoint: endpoint,
            channels: Vec::new(),
            steps: steps,
            bpm_duration: Duration::from_millis((((60f32 / bpm as f32) * 1000f32) / per_bar) as u64),
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

    /// Generates a saw-tooth wave from samples
    fn saw_wave(t:f32) -> f32 {
        t - t.floor()
    }

    /// Generates white noise from samples
    fn noise_wave(_:f32) -> f32 {
        random()
    }

    /// Add a voice channel to audact for synth playback
    pub fn channel(&mut self, freq: f32, wave: Wave, volume: f32,
                   filter: (f32, f32), seq: Vec<i32>) -> Result<bool, bool> {
        // create the sink to play from
        let sink = Sink::new(&self.endpoint);

        let wave = match wave {
            Wave::Sine => Audact::sine_wave,
            Wave::Square => Audact::square_wave,
            Wave::Saw => Audact::saw_wave,
            Wave::Noise => Audact::noise_wave,
        };

        let (hp, lp) = filter;

        let samples_rate = 44100f32;
        let data_source = (0u64..).map(move |t| {
            let freq = t as f32 * freq * PI / samples_rate; // freq
            let sample = wave(freq).max(hp).min(lp); // high & low pass
            // create the sample buffer
            SamplesBuffer::new(2, samples_rate as u32, vec![sample, sample, sample, sample])
        });


        let source = source::from_iter(data_source);
        sink.append(source.amplify(volume));
        sink.pause();

        self.channels.push((sink, seq));

        Ok(true)
    }

    /// Kick off audact to start and loop 'bars' times
    pub fn start(audact:Audact, bars: i32) {
        // grab some values from the stuct to be moved
        let steps = audact.steps;
        let bpm_duration = audact.bpm_duration;
        let tmp_voice_channels = audact.channels;

        let handle = thread::spawn(move || {
            for _ in 0 .. bars {
                // simple step sequencer
                for step in 0 .. steps {
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

            for i in 0 .. tmp_voice_channels.len() {
                tmp_voice_channels[i].0.stop();
            }
        });


        let _ = handle.join().unwrap();
    }
}
