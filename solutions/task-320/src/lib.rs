// rust-track: handwritten task-320
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 320;

pub fn leading_marks_removed_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
