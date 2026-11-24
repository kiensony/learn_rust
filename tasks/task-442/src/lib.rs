#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_equals(text: &mut String) -> () {
    let selected = text.as_str();
    text.clear();
    text.push_str(selected);
}
