// rust-track: handwritten task-489
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 489;

pub fn closing_field_equals(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit('=').next().unwrap_or_default()
    })
    .len()
}
