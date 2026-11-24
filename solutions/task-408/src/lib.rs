// rust-track: handwritten task-408
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 408;

pub fn leading_field_colon(text: &mut String) -> () {
    let selected = (text.split(':').next().unwrap_or_default()).to_owned();
    text.clear();
    text.push_str(&selected);
}
