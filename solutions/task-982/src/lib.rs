// rust-track: handwritten task-982
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 982;

pub fn leading_field_equals<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.split_once(';').map_or(text, |(data, _)| data);
                text.split('=').next().unwrap_or_default()
            })
            .len()
        })
        .collect()
}
