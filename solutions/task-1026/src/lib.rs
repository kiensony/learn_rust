// rust-track: handwritten task-1026
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1026;

pub fn marks_removed_at_both_ends_pipe<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.trim_matches('|')
        })
        .len()
    })
}
