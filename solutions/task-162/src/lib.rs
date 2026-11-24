// rust-track: handwritten task-162
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 162;

pub fn higher(first: i16, second: i16) -> i16 {
    if first > second { first } else { second }
}
