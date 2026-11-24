// rust-track: handwritten task-781
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 781;

pub fn longest_field_with_last_tie_pipe(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    longest_field_with_last_tie_pipe_impl(records)
}

fn longest_field_with_last_tie_pipe_impl(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.lines().last().unwrap_or_default();
                    text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
