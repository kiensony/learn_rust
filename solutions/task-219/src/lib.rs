// rust-track: handwritten task-219
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 219;

pub fn number_of_divisors_2(value: u8) -> u32 {
    (1..=2).filter(|d| u32::from(value) % d == 0).count() as u32
}
