#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn number_of_divisors_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    (1..=8)
        .filter(|d| u32::from(value.saturating_sub(1)) % d == 0)
        .count() as u32
}
