// rust-track: handwritten task-284
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 284;

pub fn read_a_heading_tilde(text: &str) -> String {
    text.split('~').next().unwrap_or_default().to_owned()
}
