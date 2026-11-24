// rust-track: handwritten task-187
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 187;

pub fn three_shelves_5(issued: u16, returned: u16) -> u8 {
    let value = issued.saturating_sub(returned);
    if value < 5 {
        0
    } else if value <= 7 {
        1
    } else {
        2
    }
}
