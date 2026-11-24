// rust-track: handwritten task-2560
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2560;

pub fn acknowledge(sample: Box<[u16]>) -> usize {
    sample.len()
}
