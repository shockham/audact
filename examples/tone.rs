extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes::std_note_freq;

fn main() {
    let mut audact = Audact::new(16, 100, 4f32);

    // single test tone
    audact.channel(std_note_freq(0), Wave::Sine,
                   0.7f32, (0f32, 3000f32), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]).unwrap();

    Audact::start(audact, 4);
}
