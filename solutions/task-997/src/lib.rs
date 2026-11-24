// rust-track: handwritten task-997
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 997;

pub fn remainder_after_first_mark_hash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.split_once('#').map_or("", |(_, b)| b)
            })
            .len()
        })
        .collect()
}
