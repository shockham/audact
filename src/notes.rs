/// Calculate notes from a base note and intervals
pub fn note_freq(base_freq:f32, interval:i32, note_offset:i32) -> f32 {
    base_freq * (2f32.powf(1f32 / interval as f32)).powf(note_offset as f32)
}

/// Calculate note from offset on standard scale
pub fn std_note_freq(note_offset:i32) -> f32 {
    note_freq(440f32, 12i32, note_offset)
}


#[test]
fn test_note_freq() {
    assert_eq!(note_freq(440f32, 12, 2), 493.88336);
}

#[test]
fn test_std_note_freq() {
    assert_eq!(std_note_freq(2), 493.88336);
}


/// C 2
pub const C2:f32 = 65.41;
/// C Sharp 2
pub const CS2:f32 = 69.30;
/// D 2
pub const D2:f32 = 73.42;
/// D Sharp 2
pub const DS2:f32 = 77.78;
/// E 2
pub const E2:f32 = 82.41;
/// F 2
pub const F2:f32 = 87.31;
/// F Sharp 2
pub const FS2:f32 = 92.50;
/// G 2
pub const G2:f32 = 98.00;
/// G Sharp 2
pub const GS2:f32 = 103.8;
/// A 2
pub const A2:f32 = 110.0;
/// A Sharp 2
pub const AS2:f32 = 116.5;
/// B 2
pub const B2:f32 = 123.5;
/// C 3
pub const C3:f32 = 130.8;
/// C Sharp 3
pub const CS3:f32 = 138.6;
/// D 3
pub const D3:f32 = 146.8;
/// D Sharp 3
pub const DS3:f32 = 155.6;
/// E 3
pub const E3:f32 = 164.8;
/// F 3
pub const F3:f32 = 174.6;
/// F Sharp 3
pub const FS3:f32 = 185.0;
/// G 3
pub const G3:f32 = 196.0;
/// G sharp 3
pub const GS3:f32 = 207.7;
/// A 3
pub const A3:f32 = 220.0;
/// A Sharp 3
pub const AS3:f32 = 233.1;
/// B 3
pub const B3:f32 = 246.9;
/// C 4
pub const C4:f32 = 261.63;
/// C Sharp 4
pub const CS4:f32 = 277.18;
/// D 4
pub const D4:f32 = 293.66;
/// D Sharp 4
pub const DS4:f32 = 311.13;
/// E 4
pub const E4:f32 = 329.63;
/// F 4
pub const F4:f32 = 349.23;
/// F Sharp 4
pub const FS4:f32 = 369.99;
/// G 4
pub const G4:f32 = 392.00;
/// G Sharp 4
pub const GS4:f32 = 415.30;
/// A 4
pub const A4:f32 = 440.00;
/// A Sharp 4
pub const AS4:f32 = 466.16;
/// B 4
pub const B4:f32 = 493.88;
/// C 5
pub const C5:f32 = 523.3;
/// C Sharp 5
pub const CS5:f32 = 554.4;
/// D 5
pub const D5:f32 = 587.3;
/// D Sharp 5
pub const DS5:f32 = 622.3;
/// E 5
pub const E5:f32 = 659.3;
/// F 5
pub const F5:f32 = 698.5;
/// F Sharp 5
pub const FS5:f32 = 740.0;
/// G 5
pub const G5:f32 = 784.0;
/// G Sharp 5
pub const GS5:f32 = 830.6;
/// A 5
pub const A5:f32 = 880.0;
/// A Sharp 5
pub const AS5:f32 = 932.3;
/// B 5
pub const B5:f32 = 987.8;
