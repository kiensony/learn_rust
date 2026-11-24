// rust-track: handwritten task-799
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 799;

pub fn leader(votes: &[u8]) -> Option<u8> {
    let mut counts = std::collections::BTreeMap::new();
    for &id in votes {
        *counts.entry(id).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(id, n)| (n, std::cmp::Reverse(id)))
        .map(|(id, _)| id)
}
