// rust-track: handwritten task-2870
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2870;

pub fn big_endian_registers_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_be_bytes([c[0], c[1]])))
            .collect()
    };
    samples.into_iter().rev().collect()
}
