// rust-track: handwritten task-369
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 369;

pub fn closing_field_slash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    }
    .len()
}
