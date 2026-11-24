// rust-track: handwritten task-477
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 477;

pub fn shortest_field_with_first_tie_slash(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
