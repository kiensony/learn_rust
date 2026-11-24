// rust-track: handwritten task-160
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 160;

pub fn fractional_fee_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value) * 10 / 100
}
