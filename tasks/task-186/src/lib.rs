#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn fare(age: u8) -> u8 {
    if age < 5 {
        0
    } else if age < 17 {
        2
    } else {
        5
    }
}
