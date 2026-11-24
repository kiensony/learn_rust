// rust-track: handwritten task-353
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 353;

pub fn remainder_after_first_mark_hash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split_once('#').map_or("", |(_, b)| b)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
