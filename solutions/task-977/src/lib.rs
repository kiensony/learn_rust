// rust-track: handwritten task-977
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 977;

pub fn remainder_after_first_mark_pipe<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split_once('|').map_or("", |(_, b)| b)
            })
            .len()
        })
        .collect()
}
