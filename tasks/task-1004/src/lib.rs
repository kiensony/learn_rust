#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1004;

pub fn clear<const N: usize>() -> [u8; N] {
    const SIZE: usize = N;
    [0; SIZE]
}
