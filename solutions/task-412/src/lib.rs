// rust-track: handwritten task-412
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 412;

pub fn marks_removed_at_both_ends_colon(text: &mut String) -> () {
    let selected = (text.trim_matches(':')).to_owned();
    text.clear();
    text.push_str(&selected);
}
