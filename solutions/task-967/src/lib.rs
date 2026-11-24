// rust-track: handwritten task-967
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 967;

pub fn remainder_after_first_mark_slash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.split_once('/').map_or("", |(_, b)| b)
            })
            .len()
        })
        .collect()
}
