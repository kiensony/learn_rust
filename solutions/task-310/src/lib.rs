// rust-track: handwritten task-310
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 310;

pub fn closing_field_colon(mut text: String) -> String {
    let selected = (text.rsplit(':').next().unwrap_or_default()).to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
