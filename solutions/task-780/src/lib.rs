// rust-track: handwritten task-780
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 780;

pub fn first_nonempty_field_pipe(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.lines().last().unwrap_or_default();
                    text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
