extern crate cpal;
extern crate futures;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use cpal::{ UnknownTypeBuffer, Endpoint, EventLoop, Voice };

use std::sync::Arc;

fn main() {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let event_loop = Arc::new(EventLoop::new());

    voice_channel(&endpoint,  &event_loop).play();

    (*event_loop).run();
}

fn voice_channel(endpoint: &Endpoint, event_loop: &Arc<EventLoop>) -> Voice {
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");
    let (voice, stream) = cpal::Voice::new(&endpoint, &format,
                                               &event_loop).expect("Failed to create a voice");

    let freq = 100.0;
    //let gap = Duration::from_millis(250);

    let samples_rate = format.samples_rate.0 as f32;
    let mut data_source = (0u64..).map(move |t| t as f32 * freq * 2.0 * 3.141592 / samples_rate)
        .map(move |t| t.sin());

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
