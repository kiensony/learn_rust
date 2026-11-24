// rust-track: handwritten task-130
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 130;

pub fn fractional_fee_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) * 3 / 100
}
