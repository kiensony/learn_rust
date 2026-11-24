// rust-track: handwritten task-499
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 499;

pub fn closing_field_hash(fields: (&str, &str)) -> usize {
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
        text.rsplit('#').next().unwrap_or_default()
    })
    .len()
}
