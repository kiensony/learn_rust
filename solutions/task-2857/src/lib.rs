// rust-track: handwritten task-2857
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2857;

pub fn little_endian_registers_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_little_endian_registers(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_little_endian_registers(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
        .collect()
}
