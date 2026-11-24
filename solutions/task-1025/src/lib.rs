// rust-track: handwritten task-1025
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1025;

pub fn closing_marks_removed_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.trim_end_matches('|')
        })
        .len()
    })
}
