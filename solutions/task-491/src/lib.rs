// rust-track: handwritten task-491
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 491;

pub fn closing_marks_removed_equals(fields: (&str, &str)) -> usize {
    closing_marks_removed_equals_impl(fields)
}

fn closing_marks_removed_equals_impl(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    })
    .len()
}
