#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn window(bytes: &[u8], start: usize, width: usize) -> Option<&[u8]> {
    bytes.get(start..width)
}
