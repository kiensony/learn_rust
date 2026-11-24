// rust-track: handwritten task-575
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 575;

pub fn longest_field_with_last_tie_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    }
}
