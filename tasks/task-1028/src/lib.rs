#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn slots<const N: usize>(_values: &[u16; N]) -> usize {
    std::mem::size_of::<[u16; N]>()
}
