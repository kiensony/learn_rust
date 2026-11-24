// rust-track: handwritten task-2853
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2853;

pub fn little_endian_registers_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
