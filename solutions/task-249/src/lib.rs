// rust-track: handwritten task-249
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 249;

pub fn number_of_divisors_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    (1..=8).filter(|d| u32::from(value) % d == 0).count() as u32
}
