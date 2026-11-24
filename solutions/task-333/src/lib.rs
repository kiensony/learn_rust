// rust-track: handwritten task-333
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 333;

pub fn remainder_after_first_mark_pipe(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
