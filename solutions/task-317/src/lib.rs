// rust-track: handwritten task-317
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 317;

pub fn shortest_field_with_first_tie_colon(mut text: String) -> String {
    let selected = (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
