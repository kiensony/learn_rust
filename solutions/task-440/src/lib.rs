// rust-track: handwritten task-440
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 440;

pub fn leading_marks_removed_equals(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
