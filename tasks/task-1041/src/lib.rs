#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shortest_field_with_first_tie_equals<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.split('=')
                .min_by_key(|s| s.chars().count())
                .unwrap_or_default()
        })
        .chars()
        .count()
    })
}
