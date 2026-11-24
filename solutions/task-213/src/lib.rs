// rust-track: handwritten task-213
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 213;

pub fn running_deposits_2(value: u8) -> u32 {
    (1..=u32::from(value)).map(|day| day * 2).sum()
}
