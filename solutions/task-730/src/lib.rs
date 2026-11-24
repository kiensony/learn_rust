// rust-track: handwritten task-730
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 730;

pub fn port(text: &str) -> Option<u16> {
    text.parse::<u16>().ok()
}
