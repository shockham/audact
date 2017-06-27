# audact

Simple synth and sequencing lib

Just has simple sine, square, saw and noise waveforms at the moment. Planning on putting in filters.

Usage:

```rust
extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes;

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    audact.voice_channel(notes::C3, Wave::Sine, (0f32, 1f32), vec![0,4,8,12]).unwrap();
    audact.voice_channel(notes::D3, Wave::Square, (0f32, 1f32), vec![2,6,10,14]).unwrap();

    Audact::start(audact);
}
```
