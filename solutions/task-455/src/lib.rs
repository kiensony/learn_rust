// rust-track: handwritten task-455
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 455;

pub fn first_nonempty_field_hash(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
