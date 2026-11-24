// rust-track: handwritten task-325
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 325;

pub fn first_nonempty_field_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
