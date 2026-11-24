#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn prefix_before_final_mark_colon<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    Vec::new()
}
