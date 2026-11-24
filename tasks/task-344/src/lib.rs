#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn prefix_before_final_mark_equals(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
