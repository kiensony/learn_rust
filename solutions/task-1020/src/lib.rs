// rust-track: handwritten task-1020
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1020;

pub fn longest_field_with_last_tie_slash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
    })
}
