// rust-track: handwritten task-461
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 461;

pub fn closing_marks_removed_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.trim_end_matches(':')).len()
}
