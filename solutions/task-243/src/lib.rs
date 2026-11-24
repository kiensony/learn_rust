// rust-track: handwritten task-243
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 243;

pub fn running_deposits_8(start: u8, end: u8) -> u32 {
    running_deposits_8_impl(start, end)
}

fn running_deposits_8_impl(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    (1..=u32::from(value)).map(|day| day * 8).sum()
}
