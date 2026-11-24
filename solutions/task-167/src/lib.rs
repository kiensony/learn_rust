// rust-track: handwritten task-167
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 167;

pub fn three_shelves_2(value: u16) -> u8 {
    if value < 2 {
        0
    } else if value <= 4 {
        1
    } else {
        2
    }
}
