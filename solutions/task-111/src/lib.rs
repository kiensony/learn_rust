// rust-track: handwritten task-111
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 111;

pub fn change(before: i8, after: i8) -> i16 {
    i16::from(after) - i16::from(before)
}
