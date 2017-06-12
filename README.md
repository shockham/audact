
Simple synth and sequencing lib

Usage:

```rust
extern crate audact;

use audact::system::{ Audact, Wave };

fn main() {
    let mut audact = Audact::new(16, 120, 4f32);

    audact.voice_channel(100.0, Wave::Sine, vec![0,4,8,12]).unwrap();
    audact.voice_channel(200.0, Wave::Square, vec![2,6,10,14]).unwrap();

    Audact::start(audact);
}
```
