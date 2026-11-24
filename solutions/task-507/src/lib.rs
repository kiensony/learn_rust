// rust-track: handwritten task-507
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 507;

pub fn shortest_field_with_first_tie_hash(fields: (&str, &str)) -> usize {
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
        text.split('#').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
