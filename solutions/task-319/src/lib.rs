// rust-track: handwritten task-319
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 319;

pub fn closing_field_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
