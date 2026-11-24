// rust-track: handwritten task-434
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 434;

pub fn append_reading(mut readings: Vec<i32>, next: i32) -> (Vec<i32>, Option<i32>) {
    let previous = readings.first().copied();
    readings.push(next);
    (readings, previous)
}
