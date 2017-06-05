extern crate cpal;
extern crate futures;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cpal::{ Endpoint, Format, EventLoop };

struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

struct Audact {
    endpoint: Endpoint,
    format: Format,
    event_loop: EventLoop,
}

fn main() {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");

    let audact = Audact {
        endpoint: endpoint,
        format: format,
        event_loop: cpal::EventLoop::new(),
    };

    let task = loop_channel(&audact);

    audact.event_loop.run();
}

fn loop_channel(audact:&Audact) {
    let executor = Arc::new(MyExecutor);
    let (mut voice, stream) = cpal::Voice::new(&audact.endpoint, &audact.format,
                                               &audact.event_loop).expect("Failed to create a voice");

    let freq = 100.0;
    let gap = Duration::from_millis(250);

    // Produce a sinusoid of maximum amplitude.
    let samples_rate = audact.format.samples_rate.0 as f32;
    let mut data_source = (0u64..).map(move |t| t as f32 * freq * 2.0 * 3.141592 / samples_rate)     // 440 Hz
        .map(move |t| t.sin());

    voice.play();

    let task = stream.for_each(move |buffer| -> Result<_, ()> {
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(audact.format.channels.len()).zip(&mut data_source) {
                    let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },
            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(audact.format.channels.len()).zip(&mut data_source) {
                    let value = (value * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },
            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(audact.format.channels.len()).zip(&mut data_source) {
                    for out in sample.iter_mut() { *out = value; }
                }
            },
        };

        Ok(())
    });
    
    task::spawn(task).execute(executor);
}
