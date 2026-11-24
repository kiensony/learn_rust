// rust-track: handwritten task-1016
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1016;

pub fn marks_removed_at_both_ends_slash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.trim_matches('/')
        })
        .len()
    })
}
