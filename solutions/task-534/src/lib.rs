// rust-track: handwritten task-534
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 534;

pub fn preview(text: &str, max_bytes: usize) -> &str {
    let mut end = text.len().min(max_bytes);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}
