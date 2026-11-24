#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shortest_field_with_first_tie_pipe(mut text: String) -> String {
    let moved = text;
    text.push_str(&moved);
    text
}
