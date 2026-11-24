// rust-track: handwritten task-409
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 409;

pub fn closing_field_colon(text: &mut String) -> () {
    let selected = (text.rsplit(':').next().unwrap_or_default()).to_owned();
    text.clear();
    text.push_str(&selected);
}
