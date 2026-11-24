// rust-track: handwritten task-462
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 462;

pub fn marks_removed_at_both_ends_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.trim_matches(':')).len()
}
