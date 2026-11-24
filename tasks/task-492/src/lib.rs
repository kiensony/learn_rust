#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_equals(fields: (&str, &str)) -> usize {
    let text = if fields.0.chars().count() >= fields.1.chars().count() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_matches('=')
    })
    .chars()
    .count()
}
