// rust-track: handwritten task-347
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 347;

pub fn shortest_field_with_first_tie_equals(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
