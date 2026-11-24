#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn first_nonempty_field_slash(text: &mut String) -> () {
    let selected = text.as_str();
    text.clear();
    text.push_str(selected);
}
