#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn leading_marks_removed_hash(fields: (&str, &str)) -> usize {
    let text = if fields.0.chars().count() >= fields.1.chars().count() {
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
        text.trim_start_matches('#')
    })
    .chars()
    .count()
}
