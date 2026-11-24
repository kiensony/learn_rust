// rust-track: handwritten task-239
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 239;

pub fn number_of_divisors_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=5).filter(|d| u32::from(value) % d == 0).count() as u32
}
