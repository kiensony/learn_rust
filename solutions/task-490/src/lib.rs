// rust-track: handwritten task-490
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 490;

pub fn leading_marks_removed_equals(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    })
    .len()
}
