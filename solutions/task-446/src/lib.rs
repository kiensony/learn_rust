// rust-track: handwritten task-446
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 446;

pub fn longest_field_with_last_tie_equals(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
