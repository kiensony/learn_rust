// rust-track: handwritten task-401
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 401;

pub fn closing_marks_removed_hash(text: &String) -> usize {
    closing_marks_removed_hash_impl(text)
}

fn closing_marks_removed_hash_impl(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    }
    .len()
}
