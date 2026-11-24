// rust-track: handwritten task-140
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 140;

pub fn fractional_fee_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value) * 5 / 100
}
