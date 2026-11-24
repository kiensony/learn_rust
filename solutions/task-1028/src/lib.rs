// rust-track: handwritten task-1028
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1028;

pub fn slots<const N: usize>(_values: &[u16; N]) -> usize {
    N
}
