// rust-track: handwritten task-996
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 996;

pub fn marks_removed_at_both_ends_hash<'a>(
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
                text.trim_matches('#')
            })
            .len()
        })
        .collect()
}
