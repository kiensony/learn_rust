// rust-track: handwritten task-259
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 259;

pub fn number_of_divisors_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    (1..=10).filter(|d| u32::from(value) % d == 0).count() as u32
}
