// rust-track: handwritten task-496
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 496;

pub fn longest_field_with_last_tie_equals(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
