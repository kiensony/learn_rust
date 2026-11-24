#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn count_chimes_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=u32::from(value.saturating_sub(1)))
        .filter(|i| i % 5 == 0)
        .count() as u32
}
