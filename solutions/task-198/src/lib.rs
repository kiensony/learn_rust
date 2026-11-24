// rust-track: handwritten task-198
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 198;

pub fn priority_fee_8(start: u16, end: u16) -> u8 {
    let value = start.abs_diff(end);
    if value == 0 {
        0
    } else if value <= 10 {
        8
    } else {
        16
    }
}
