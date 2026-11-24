// rust-track: handwritten task-976
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 976;

pub fn marks_removed_at_both_ends_pipe<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.trim_matches('|')
            })
            .len()
        })
        .collect()
}
