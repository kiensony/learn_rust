// rust-track: handwritten task-484
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 484;

pub fn window(bytes: &[u8], start: usize, width: usize) -> Option<&[u8]> {
    bytes.get(start..start.checked_add(width)?)
}
