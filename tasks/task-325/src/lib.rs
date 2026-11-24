#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn first_nonempty_field_slash(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
