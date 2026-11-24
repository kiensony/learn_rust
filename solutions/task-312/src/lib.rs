// rust-track: handwritten task-312
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 312;

pub fn closing_marks_removed_colon(mut text: String) -> String {
    let selected = (text.trim_end_matches(':')).to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
