// rust-track: handwritten task-472
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 472;

pub fn marks_removed_at_both_ends_slash(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    })
    .len()
}
