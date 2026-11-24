// rust-track: handwritten task-503
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 503;

pub fn remainder_after_first_mark_hash(fields: (&str, &str)) -> usize {
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
        text.split_once('#').map_or("", |(_, b)| b)
    })
    .len()
}
