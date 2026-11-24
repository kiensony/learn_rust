// rust-track: handwritten task-2852
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2852;

pub fn little_endian_registers_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
