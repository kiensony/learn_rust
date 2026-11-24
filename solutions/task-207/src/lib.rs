// rust-track: handwritten task-207
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 207;

pub fn three_shelves_10(requested: u16, available: u16) -> u8 {
    let value = requested.min(available);
    if value < 10 {
        0
    } else if value <= 12 {
        1
    } else {
        2
    }
}
