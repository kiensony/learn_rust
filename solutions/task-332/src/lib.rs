// rust-track: handwritten task-332
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 332;

pub fn marks_removed_at_both_ends_pipe(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
