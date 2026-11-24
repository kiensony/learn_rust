// rust-track: handwritten task-221
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 221;

pub fn sum_chimes_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    (1..=u32::from(value)).filter(|i| i % 3 == 0).sum()
}
