// rust-track: handwritten task-968
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 968;

pub fn prefix_before_final_mark_slash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    prefix_before_final_mark_slash_impl(records)
}

fn prefix_before_final_mark_slash_impl<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.rsplit_once('/').map_or("", |(a, _)| a)
            })
            .len()
        })
        .collect()
}
