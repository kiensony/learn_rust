// rust-track: handwritten task-404
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 404;

pub fn has_header(bytes: &[u8], header: &[u8]) -> bool {
    bytes.starts_with(header)
}
