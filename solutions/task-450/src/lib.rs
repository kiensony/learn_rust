// rust-track: handwritten task-450
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 450;

pub fn leading_marks_removed_hash(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_start_matches('#')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
