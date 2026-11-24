#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn preview(text: &str, max_bytes: usize) -> &str {
    &text[..text.len().min(max_bytes)]
}
