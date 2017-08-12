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
struct Processing {
    /// Volume
    volume: f32,
    /// Filter
    filter: (f32, f32),
    /// Attack
    attack: Duration,
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
            sample_rate: 44100u32,
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
        (random::<f32>() * 2f32) - 1f32
    }

    /// Add a voice channel to audact for synth playback
    pub fn channel(&mut self, freq: f32, wave: Wave, volume: f32,
                   filter: (f32, f32), attack: f32, seq: Vec<i32>) {
        // create the sink to play from
        let sink = Sink::new(&self.endpoint);
        sink.pause();


        // Calculate the number of samples needed per step
        let samples_rate = self.sample_rate as f32;
        let subsecs = self.bpm_duration.subsec_nanos() as f32 / 100000000f32;
        let samples_needed = samples_rate * ((self.bpm_duration.as_secs() as f32 + subsecs) / 4f32);

        // Create the basic waveform samples
        let source:Vec<f32> = (0u64..samples_needed as u64).map(move |t| {
            let freq = t as f32 * freq * PI / samples_rate; // freq

            match wave {
                Wave::Sine => Audact::sine_wave(freq),
                Wave::Square => Audact::square_wave(freq),
                Wave::Saw => Audact::saw_wave(freq),
                Wave::Noise => Audact::noise_wave(freq),
            }
        }).collect();

        // Create the processing chain and channel
        let attack = Duration::from_millis(attack as u64);
        let processing = Processing { volume, filter, attack };
        let channel = Channel { sink, seq, source, processing };

        self.channels.push(channel);
    }

    /// Kick off audact to start and loop 'bars' times
    pub fn start(audact:Audact, bars: i32) {
        // grab some values from the stuct to be moved
        let steps = audact.steps;
        let bpm_duration = audact.bpm_duration;
        let tmp_voice_channels = audact.channels;
        let sample_rate = audact.sample_rate;

        let handle = thread::spawn(move || {
            for _ in 0 .. bars {
                // simple step sequencer
                for step in 0 .. steps {
                    for i in 0 .. tmp_voice_channels.len() {
                        if let Ok(_) = tmp_voice_channels[i].seq.binary_search(&step) {
                            let chan = &tmp_voice_channels[i];
                            // create the Sample buffer
                            let samples: Vec<SamplesBuffer<f32>> = chan.source.iter()
                                .map(|&s| SamplesBuffer::new(2, sample_rate, vec![s]))
                                .collect();
                            // processing values
                            let vol = chan.processing.volume;
                            let (_, lp) = chan.processing.filter;
                            let attack = chan.processing.attack;
                            // create the source
                            let source = source::from_iter(samples)
                                .fade_in(attack)
                                .low_pass(lp as u32)
                                .amplify(vol);
                            // add source to sink queue
                            chan.sink.append(source);

                            if tmp_voice_channels[i].sink.is_paused() {
                                tmp_voice_channels[i].sink.play();
                            }
                        } else {
                            //tmp_voice_channels[i].sink.stop();
                            tmp_voice_channels[i].sink.pause();
                        }
                    }
                    thread::sleep(bpm_duration);
                }
            }

            for i in 0 .. tmp_voice_channels.len() {
                tmp_voice_channels[i].sink.stop();
            }
        });


        let _ = handle.join().unwrap();
    }
}
