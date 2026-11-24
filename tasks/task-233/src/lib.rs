#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn running_deposits_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=u32::from(value.saturating_sub(1)))
        .map(|day| day * 5)
        .sum()
}
