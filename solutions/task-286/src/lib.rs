// rust-track: handwritten task-286
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 286;

pub fn blank_notice_tilde(text: &str) -> String {
    let s = text.trim();
    if s.is_empty() { "~".into() } else { s.into() }
}
