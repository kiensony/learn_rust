#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn unique(sessions: &[&[u8]]) -> usize {
    sessions.iter().map(|s| s.len()).sum()
}
