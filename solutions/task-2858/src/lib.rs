// rust-track: handwritten task-2858
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2858;

pub fn little_endian_registers_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
