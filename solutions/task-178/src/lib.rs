// rust-track: handwritten task-178
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 178;

pub fn priority_fee_3(first: u16, second: u16) -> u8 {
    let value = first.saturating_add(second);
    if value == 0 {
        0
    } else if value <= 5 {
        3
    } else {
        6
    }
}
