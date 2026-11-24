// rust-track: handwritten task-786
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 786;

pub fn closing_marks_removed_equals(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    closing_marks_removed_equals_impl(records)
}

fn closing_marks_removed_equals_impl(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.split_once(';').map_or(text, |(data, _)| data);
                    text.trim_end_matches('=')
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
