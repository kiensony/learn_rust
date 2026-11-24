// rust-track: handwritten task-1030
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1030;

pub fn longest_field_with_last_tie_pipe<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    longest_field_with_last_tie_pipe_impl(records)
}

fn longest_field_with_last_tie_pipe_impl<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
    })
}
