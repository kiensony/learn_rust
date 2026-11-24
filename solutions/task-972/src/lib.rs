// rust-track: handwritten task-972
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 972;

pub fn leading_field_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split('|').next().unwrap_or_default()
            })
            .len()
        })
        .collect()
}
