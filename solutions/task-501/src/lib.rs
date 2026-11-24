// rust-track: handwritten task-501
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 501;

pub fn closing_marks_removed_hash(fields: (&str, &str)) -> usize {
    closing_marks_removed_hash_impl(fields)
}

fn closing_marks_removed_hash_impl(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
    .len()
}
