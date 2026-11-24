#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn three_shelves_3(first: u16, second: u16) -> u8 {
    let value = first.saturating_add(second);
    let value = value.saturating_sub(1);
    if value < 3 {
        0
    } else if value <= 5 {
        1
    } else {
        2
    }
}
