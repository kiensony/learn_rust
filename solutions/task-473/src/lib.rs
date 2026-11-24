// rust-track: handwritten task-473
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 473;

pub fn remainder_after_first_mark_slash(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    })
    .len()
}
