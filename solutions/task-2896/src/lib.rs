// rust-track: handwritten task-2896
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2896;

pub fn little_endian_registers_threshold_crossing(
    input: &[u8],
    limit: u32,
) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
