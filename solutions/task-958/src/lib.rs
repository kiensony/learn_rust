// rust-track: handwritten task-958
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 958;

pub fn prefix_before_final_mark_colon<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    prefix_before_final_mark_colon_impl(records)
}

fn prefix_before_final_mark_colon_impl<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.rsplit_once(':').map_or("", |(a, _)| a)).len())
        .collect()
}
