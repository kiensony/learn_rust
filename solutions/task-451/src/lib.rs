// rust-track: handwritten task-451
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 451;

pub fn closing_marks_removed_hash(text: &mut String) -> () {
    closing_marks_removed_hash_impl(text)
}

fn closing_marks_removed_hash_impl(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
