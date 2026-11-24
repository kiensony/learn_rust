// rust-track: handwritten task-177
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 177;

pub fn three_shelves_3(first: u16, second: u16) -> u8 {
    let value = first.saturating_add(second);
    if value < 3 {
        0
    } else if value <= 5 {
        1
    } else {
        2
    }
}
