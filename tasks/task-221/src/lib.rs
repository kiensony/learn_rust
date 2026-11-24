#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn sum_chimes_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    (1..=u32::from(value.saturating_sub(1)))
        .filter(|i| i % 3 == 0)
        .sum()
}
