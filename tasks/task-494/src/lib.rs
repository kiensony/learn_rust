#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn prefix_before_final_mark_equals(fields: (&str, &str)) -> usize {
    let text = if fields.0.chars().count() >= fields.1.chars().count() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    })
    .chars()
    .count()
}
