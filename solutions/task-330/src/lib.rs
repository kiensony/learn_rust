// rust-track: handwritten task-330
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 330;

pub fn leading_marks_removed_pipe(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_start_matches('|')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
