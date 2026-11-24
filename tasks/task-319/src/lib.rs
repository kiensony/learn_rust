#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn closing_field_slash(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
