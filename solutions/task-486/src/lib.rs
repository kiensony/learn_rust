// rust-track: handwritten task-486
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 486;

pub fn longest_field_with_last_tie_pipe(fields: (&str, &str)) -> usize {
    longest_field_with_last_tie_pipe_impl(fields)
}

fn longest_field_with_last_tie_pipe_impl(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
