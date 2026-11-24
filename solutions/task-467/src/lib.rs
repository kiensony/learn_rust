// rust-track: handwritten task-467
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 467;

pub fn shortest_field_with_first_tie_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len()
}
