// rust-track: handwritten task-102
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 102;

pub fn opening_balance_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value) + 10
}
