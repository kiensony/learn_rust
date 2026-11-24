// rust-track: handwritten task-229
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 229;

pub fn number_of_divisors_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    (1..=3).filter(|d| u32::from(value) % d == 0).count() as u32
}
