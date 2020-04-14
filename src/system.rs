use rodio;
use rodio::buffer::SamplesBuffer;
use rodio::source;
use rodio::Device;
use rodio::Sink;
use rodio::Source;

use std::f32::consts::PI;
use std::time::Duration;

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
    endpoint: Device,
    /// Vec of voice channels that audact will play
    channels: Vec<Channel>,
    /// The number of steps for the sequencer
    steps: i32,
    /// Sample rate
    sample_rate: u32,
    /// Samples needed per step
    total_samples_needed: f32,
}

/// Stuct to represent a channel
struct Channel {
    /// The Sink that the channel plays from
    sink: Sink,
    /// The samples the channel plays
    source: Vec<f32>,
    /// Processing for the channel
    processing: Processing,
}

/// Represents processing values on a channel
#[derive(Builder, Clone, Copy)]
#[builder(default)]
pub struct Processing {
    /// Volume
    gain: f32,
    /// Filter
    filter: (f32, f32),
    /// Attack
    attack: Duration,
    /// Reverb
    reverb: (Duration, f32),
}

impl Default for Processing {
    fn default() -> Self {
        Processing {
            gain: 1f32,
            filter: (0f32, 5000f32),
            attack: Duration::from_millis(0u64),
            reverb: (Duration::from_millis(0), 0f32),
        }
    }
}

/// implementation for the audact struct
impl Audact {
    /// Creates a new instance of audact
    pub fn new(steps: i32, bpm: i32, per_bar: f32) -> Audact {
        let endpoint = rodio::default_output_device().unwrap();
        // Sample rate and step duration
        let sample_rate = 44100f32;
        let bpm_duration =
            Duration::from_millis((((60f32 / bpm as f32) * 1000f32) / per_bar) as u64);
        // Calculate the number of samples needed per step
        let subsecs = bpm_duration.subsec_nanos() as f32 / 100_000_000f32;
        let samples_needed =
            sample_rate * ((bpm_duration.as_secs() as f32 + subsecs) / 4f32) * 0.8f32;
        let total_samples_needed = samples_needed * steps as f32;
        // Create and return instance
        Audact {
            endpoint,
            channels: Vec::new(),
            steps,
            sample_rate: sample_rate as u32,
            total_samples_needed,
        }
    }

    /// Generates a sine wave from samples
    fn sine_wave(t: f32) -> f32 {
        t.sin()
    }

    /// Generates a square wave from samples
    fn square_wave(t: f32) -> f32 {
        t.sin().round()
    }

    /// Generates a saw-tooth wave from samples
    fn saw_wave(t: f32) -> f32 {
        t - t.floor()
    }

    /// Generates white noise from samples
    fn noise_wave(_: f32) -> f32 {
        (random::<f32>() * 2f32) - 1f32
    }

    /// Add a voice channel to audact for synth playback
    pub fn channel(&mut self, wave: Wave, volume: f32, processing: Processing, seq: Vec<f32>) {
        // create the sink to play from
        let sink = Sink::new(&self.endpoint);
        sink.pause();
        sink.set_volume(volume);

        let sample_rate = self.sample_rate as f32;
        let steps = self.steps as f32;
        let total_samples_needed = self.total_samples_needed;

        // Create the basic waveform samples
        let mut source: Vec<f32> = (0u64..self.total_samples_needed as u64)
            .map(move |t| {
                // Silence if not playing in this step
                let s_t = total_samples_needed / t as f32;
                let freq = seq[(steps / s_t).floor() as usize];
                if freq == 0f32 {
                    return 0f32;
                }
                // Calc the freq for the wave
                let freq = t as f32 * freq * PI / sample_rate;
                // Call the wave gen fn
                match wave {
                    Wave::Sine => Audact::sine_wave(freq),
                    Wave::Square => Audact::square_wave(freq),
                    Wave::Saw => Audact::saw_wave(freq),
                    Wave::Noise => Audact::noise_wave(freq),
                }
            })
            .collect();

        // Get rid of clicks by interpolating vol changes
        // probably smarter ways of doing this
        fn smooth(source: &mut Vec<f32>) {
            let mut prev_sample = 0f32;
            for s in source.iter_mut() {
                if *s == 0f32 {
                    *s = prev_sample * 0.99;
                }
                prev_sample = *s;
            }
        }
        // smooth note off
        smooth(&mut source);
        source.reverse();
        // reverse to smooth note on then return to original
        smooth(&mut source);
        source.reverse();

        // Create the processing chain and channel
        let channel = Channel {
            sink,
            source,
            processing,
        };

        self.channels.push(channel);
    }

    /// Kick off audact to start and loop 'bars' times
    pub fn start(&mut self, bars: i32) {
        // grab some values from the stuct to be moved
        let tmp_voice_channels = &self.channels;
        let sample_rate = self.sample_rate;
        // The repeats of the sequence
        for _ in 0..bars {
            for chan in tmp_voice_channels {
                // create buffer
                let samples = chan.source.clone();
                let sample_buffer = vec![SamplesBuffer::new(2, sample_rate, samples)];
                // create the source
                let source = source::from_iter(sample_buffer)
                    .buffered()
                    .fade_in(chan.processing.attack)
                    .low_pass(chan.processing.filter.1 as u32)
                    .reverb(chan.processing.reverb.0, chan.processing.reverb.1)
                    .amplify(chan.processing.gain);
                // add source to sink queue
                chan.sink.append(source);
            }
        }
        // Play all the channels
        for chan in tmp_voice_channels {
            chan.sink.play();
        }
        // Sleep until the end of the sequence
        for chan in tmp_voice_channels {
            chan.sink.sleep_until_end();
        }
    }
}
