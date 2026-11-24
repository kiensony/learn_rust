// rust-track: handwritten task-759
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 759;

pub fn prefix_before_final_mark_colon(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    prefix_before_final_mark_colon_impl(records)
}

fn prefix_before_final_mark_colon_impl(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry((text.rsplit_once(':').map_or("", |(a, _)| a)).to_owned())
            .or_insert(0) += 1;
    }
    counts
}
