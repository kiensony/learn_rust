// rust-track: handwritten task-438
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 438;

pub fn leading_field_equals(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').next().unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
