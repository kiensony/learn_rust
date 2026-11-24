// rust-track: handwritten task-411
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 411;

pub fn closing_marks_removed_colon(text: &mut String) -> () {
    let selected = (text.trim_end_matches(':')).to_owned();
    text.clear();
    text.push_str(&selected);
}
