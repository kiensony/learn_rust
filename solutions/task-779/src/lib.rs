// rust-track: handwritten task-779
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 779;

pub fn unique(sessions: &[&[u8]]) -> usize {
    sessions
        .iter()
        .flat_map(|s| s.iter().copied())
        .collect::<std::collections::BTreeSet<_>>()
        .len()
}
