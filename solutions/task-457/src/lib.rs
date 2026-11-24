// rust-track: handwritten task-457
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 457;

pub fn shortest_field_with_first_tie_hash(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
