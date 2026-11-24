// rust-track: handwritten task-208
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 208;

pub fn priority_fee_10(requested: u16, available: u16) -> u8 {
    let value = requested.min(available);
    if value == 0 {
        0
    } else if value <= 12 {
        10
    } else {
        20
    }
}
