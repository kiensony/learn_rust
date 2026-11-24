// rust-track: handwritten task-331
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 331;

pub fn closing_marks_removed_pipe(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
