// rust-track: handwritten task-966
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 966;

pub fn marks_removed_at_both_ends_slash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.trim_matches('/')
            })
            .len()
        })
        .collect()
}
