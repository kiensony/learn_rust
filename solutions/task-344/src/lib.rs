// rust-track: handwritten task-344
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 344;

pub fn prefix_before_final_mark_equals(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
