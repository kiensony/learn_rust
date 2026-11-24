#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn longest_field_with_last_tie_equals(text: &mut String) -> () {
    let selected = text.as_str();
    text.clear();
    text.push_str(selected);
}
