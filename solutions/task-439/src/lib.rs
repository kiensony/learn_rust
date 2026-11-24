// rust-track: handwritten task-439
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 439;

pub fn closing_field_equals(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit('=').next().unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
