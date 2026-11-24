// rust-track: handwritten task-296
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 296;

pub fn blank_notice_plus(text: &str) -> String {
    let s = text.trim();
    if s.is_empty() { "+".into() } else { s.into() }
}
