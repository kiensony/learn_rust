#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn append_reading(mut readings: Vec<i32>, next: i32) -> (Vec<i32>, Option<i32>) {
    let before = readings.first();
    readings.push(next);
    let previous = before.copied();
    (readings, previous)
}
