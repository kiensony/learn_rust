// rust-track: handwritten task-974
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 974;

pub fn leading_marks_removed_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.trim_start_matches('|')
            })
            .len()
        })
        .collect()
}
