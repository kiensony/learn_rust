// rust-track: handwritten task-1004
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1004;

pub fn clear<const N: usize>() -> [u8; N] {
    [0; N]
}
