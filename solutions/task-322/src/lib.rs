// rust-track: handwritten task-322
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 322;

pub fn marks_removed_at_both_ends_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
