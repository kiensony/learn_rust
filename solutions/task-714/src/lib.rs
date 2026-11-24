// rust-track: handwritten task-714
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 714;

pub fn difference(first: Option<i16>, second: Option<i16>) -> Option<i32> {
    Some(i32::from(second?) - i32::from(first?))
}
