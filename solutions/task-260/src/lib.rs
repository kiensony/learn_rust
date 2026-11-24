// rust-track: handwritten task-260
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 260;

pub fn width(label: &str) -> usize {
    label.len()
}
