// rust-track: handwritten task-432
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 432;

pub fn marks_removed_at_both_ends_pipe(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
