// rust-track: handwritten task-313
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 313;

pub fn marks_removed_at_both_ends_colon(mut text: String) -> String {
    let selected = (text.trim_matches(':')).to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
