// rust-track: handwritten task-356
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 356;

pub fn longest_field_with_last_tie_hash(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
