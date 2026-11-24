#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn fractional_fee_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value.saturating_sub(1)) * 8 / 100
}
