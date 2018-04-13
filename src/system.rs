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
    channels: Vec<Channel>,
    /// The number of steps for the sequencer
    steps: i32,
    /// The duraction that determines the bpm
    bpm_duration: Duration,
    /// Sample rate
    sample_rate: u32,
    /// Samples needed per step
    samples_needed: f32,
}

/// Stuct to represent a channel
struct Channel {
    /// The Sink that the channel plays from
    sink: Sink,
    /// The sequence that the channel plays
    seq: Vec<i32>,
    /// The samples the channel plays
    source: Vec<f32>,
    /// Processing for the channel
    processing: Processing,
}

/// Represents processing values on a channel
#[derive(Builder, Clone, Copy)]
pub struct Processing {
    /// Volume
    #[builder(default = "1f32")]
    gain: f32,
    /// Filter
    #[builder(default = "(0f32, 5000f32)")]
    filter: (f32, f32),
    /// Attack
    #[builder(default = "Duration::from_millis(0u64)")]
    attack: Duration,
}

/// implementation for the audact struct
impl Audact {
    /// Creates a new instance of audact
    pub fn new(steps: i32, bpm: i32, per_bar: f32) -> Audact {
        let endpoint = rodio::default_endpoint().unwrap();
        // Sample rate and step duration
        let samples_rate = 44100f32;
        let bpm_duration =
            Duration::from_millis((((60f32 / bpm as f32) * 1000f32) / per_bar) as u64);
        // Calculate the number of samples needed per step
        let subsecs = bpm_duration.subsec_nanos() as f32 / 100_000_000f32;
        let samples_needed = samples_rate * ((bpm_duration.as_secs() as f32 + subsecs) / 4f32) *
            0.8f32;
        // Create and return instance
        Audact {
            endpoint,
            channels: Vec::new(),
            steps,
            bpm_duration,
            sample_rate: samples_rate as u32,
            samples_needed,
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
    pub fn channel(
        &mut self,
        freq: f32,
        wave: Wave,
        volume: f32,
        processing: Processing,
        seq: Vec<i32>,
    ) {
        // create the sink to play from
        let mut sink = Sink::new(&self.endpoint);
        sink.pause();
        sink.set_volume(volume);

        let samples_rate = self.sample_rate as f32;

        // Create the basic waveform samples
        let source: Vec<f32> = (0u64..self.samples_needed as u64)
            .map(move |t| {
                // Calc the freq for the wave
                let freq = t as f32 * freq * PI / samples_rate; // freq
                // Call the wave gen fn
                match wave {
                    Wave::Sine => Audact::sine_wave(freq),
                    Wave::Square => Audact::square_wave(freq),
                    Wave::Saw => Audact::saw_wave(freq),
                    Wave::Noise => Audact::noise_wave(freq),
                }
            })
            .collect();

        // Create the processing chain and channel
        let channel = Channel {
            sink,
            seq,
            source,
            processing,
        };

        self.channels.push(channel);
    }

    /// Kick off audact to start and loop 'bars' times
    pub fn start(&mut self, bars: i32) {
        // grab some values from the stuct to be moved
        let steps = self.steps;
        let bpm_duration = self.bpm_duration;
        let tmp_voice_channels = &self.channels;
        let sample_rate = self.sample_rate;
        let samples_needed = self.samples_needed as usize;
        // The repeats of the sequence
        for _ in 0..bars {
            // simple step sequencer
            for step in 0..steps {
                for chan in tmp_voice_channels {
                    // Check if the channel is triggered this step and get source samples or silence
                    let samples = if let Ok(_) = chan.seq.binary_search(&step) {
                        chan.source.clone()
                    } else {
                        vec![0f32; samples_needed]
                    };
                    // create buffer
                    let sample_buffer = vec![SamplesBuffer::new(2, sample_rate, samples)];
                    // create the source
                    let source = source::from_iter(sample_buffer)
                        .fade_in(chan.processing.attack)
                        .low_pass(chan.processing.filter.1 as u32)
                        .amplify(chan.processing.gain);
                    // add source to sink queue
                    chan.sink.append(source);
                    // Play the channel
                    chan.sink.play();
                }
            }
        }
        // Sleep until the end of the sequence
        thread::sleep(bpm_duration * 16u32);
        // Stop all the channels once they sequence has finished
        for chan in tmp_voice_channels {
            chan.sink.stop();
        }
    }
}
