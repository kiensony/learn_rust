// rust-track: handwritten task-476
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 476;

pub fn longest_field_with_last_tie_slash(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
