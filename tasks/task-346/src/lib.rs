#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn longest_field_with_last_tie_equals(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
