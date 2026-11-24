// rust-track: handwritten task-452
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 452;

pub fn marks_removed_at_both_ends_hash(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_matches('#')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
