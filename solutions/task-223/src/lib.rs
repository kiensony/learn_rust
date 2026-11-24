// rust-track: handwritten task-223
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 223;

pub fn running_deposits_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    (1..=u32::from(value)).map(|day| day * 3).sum()
}
