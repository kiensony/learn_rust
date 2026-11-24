#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_equals<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.trim_matches('=')
        })
        .chars()
        .count()
    })
}
