// rust-track: handwritten task-995
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 995;

pub fn closing_marks_removed_hash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    closing_marks_removed_hash_impl(records)
}

fn closing_marks_removed_hash_impl<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.trim_end_matches('#')
            })
            .len()
        })
        .collect()
}
