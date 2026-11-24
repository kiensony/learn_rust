// rust-track: handwritten task-963
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 963;

pub fn closing_field_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.rsplit('/').next().unwrap_or_default()
            })
            .len()
        })
        .collect()
}
