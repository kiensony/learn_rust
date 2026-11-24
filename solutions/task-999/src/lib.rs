// rust-track: handwritten task-999
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 999;

pub fn first_nonempty_field_hash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.split('#').find(|s| !s.is_empty()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
