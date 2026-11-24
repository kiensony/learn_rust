// rust-track: handwritten task-568
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 568;

pub fn closing_field_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    }
}
