// rust-track: handwritten task-2866
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2866;

pub fn big_endian_registers_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_be_bytes([c[0], c[1]])))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
