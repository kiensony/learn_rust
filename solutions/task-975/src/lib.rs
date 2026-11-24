// rust-track: handwritten task-975
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 975;

pub fn closing_marks_removed_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.trim_end_matches('|')
            })
            .len()
        })
        .collect()
}
