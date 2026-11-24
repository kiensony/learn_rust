// rust-track: handwritten task-197
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 197;

pub fn three_shelves_8(start: u16, end: u16) -> u8 {
    let value = start.abs_diff(end);
    if value < 8 {
        0
    } else if value <= 10 {
        1
    } else {
        2
    }
}
