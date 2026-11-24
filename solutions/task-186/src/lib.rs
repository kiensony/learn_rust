// rust-track: handwritten task-186
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 186;

pub fn fare(age: u8) -> u8 {
    if age < 5 {
        0
    } else if age <= 17 {
        2
    } else {
        5
    }
}
