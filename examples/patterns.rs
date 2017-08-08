extern crate audact;

use audact::system::{ Audact, Wave };
use audact::notes::std_note_freq;

fn main() {
    let mut pattern_1 = Audact::new(16, 100, 4f32);
    pattern_1.channel(std_note_freq(0), Wave::Sine,
                   0.7f32, (0f32, 2500f32), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);


    let mut pattern_2 = Audact::new(16, 100, 4f32);
    pattern_2.channel(std_note_freq(4), Wave::Sine,
                   0.7f32, (0f32, 2500f32), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);

    // play the patterns one after another
    Audact::start(pattern_1, 1);
    Audact::start(pattern_2, 1);
}
