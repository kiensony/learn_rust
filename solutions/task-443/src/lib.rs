// rust-track: handwritten task-443
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 443;

pub fn remainder_after_first_mark_equals(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
