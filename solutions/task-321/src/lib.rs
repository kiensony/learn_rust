// rust-track: handwritten task-321
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 321;

pub fn closing_marks_removed_slash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
