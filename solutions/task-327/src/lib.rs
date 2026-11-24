// rust-track: handwritten task-327
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 327;

pub fn shortest_field_with_first_tie_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
