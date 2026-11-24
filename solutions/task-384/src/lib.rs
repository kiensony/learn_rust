// rust-track: handwritten task-384
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 384;

pub fn mirror_count(data: &[u8]) -> usize {
    data.iter()
        .zip(data.iter().rev())
        .take(data.len() / 2)
        .filter(|(a, b)| a == b)
        .count()
}
