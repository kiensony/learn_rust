// rust-track: handwritten task-732
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 732;

pub fn longest_field_with_last_tie_pipe(record: Option<&str>) -> Option<u16> {
    longest_field_with_last_tie_pipe_impl(record)
}

fn longest_field_with_last_tie_pipe_impl(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
