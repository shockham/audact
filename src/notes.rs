/// Calculate notes from a base note and intervals
pub fn note_freq(base_freq: f32, interval: i32, note_offset: i32) -> f32 {
    base_freq * (2f32.powf(1f32 / interval as f32)).powf(note_offset as f32)
}

/// Calculate note from offset on standard scale
pub fn std_note_freq(note_offset: i32) -> f32 {
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
