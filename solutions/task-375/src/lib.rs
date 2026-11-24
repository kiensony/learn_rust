// rust-track: handwritten task-375
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 375;

pub fn first_nonempty_field_slash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
    }
    .len()
}
