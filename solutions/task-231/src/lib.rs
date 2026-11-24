// rust-track: handwritten task-231
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 231;

pub fn sum_chimes_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=u32::from(value)).filter(|i| i % 5 == 0).sum()
}
