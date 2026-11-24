// rust-track: handwritten task-463
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 463;

pub fn remainder_after_first_mark_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.split_once(':').map_or("", |(_, b)| b)).len()
}
