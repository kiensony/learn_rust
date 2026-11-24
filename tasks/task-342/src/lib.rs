#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_equals(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
