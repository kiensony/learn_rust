// rust-track: handwritten task-478
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 478;

pub fn leading_field_pipe(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    })
    .len()
}
