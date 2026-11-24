// rust-track: handwritten task-413
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 413;

pub fn remainder_after_first_mark_colon(text: &mut String) -> () {
    let selected = (text.split_once(':').map_or("", |(_, b)| b)).to_owned();
    text.clear();
    text.push_str(&selected);
}
