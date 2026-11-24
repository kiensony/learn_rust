// rust-track: handwritten task-008
pub const TRACK_PROOF: u32 = 0x52535400 | 8;

use std::collections::BTreeMap;

pub fn count_words(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_ascii_whitespace() {
        *counts.entry(word.to_ascii_lowercase()).or_default() += 1;
    }
    counts
}
