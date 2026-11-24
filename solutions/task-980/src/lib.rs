// rust-track: handwritten task-980
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 980;

pub fn longest_field_with_last_tie_pipe<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    longest_field_with_last_tie_pipe_impl(records)
}

fn longest_field_with_last_tie_pipe_impl<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
