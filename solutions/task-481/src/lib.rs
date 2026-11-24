// rust-track: handwritten task-481
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 481;

pub fn closing_marks_removed_pipe(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    })
    .len()
}
