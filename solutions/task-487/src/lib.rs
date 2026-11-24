// rust-track: handwritten task-487
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 487;

pub fn shortest_field_with_first_tie_pipe(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
