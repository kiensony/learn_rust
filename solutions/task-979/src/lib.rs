// rust-track: handwritten task-979
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 979;

pub fn first_nonempty_field_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
