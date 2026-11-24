// rust-track: handwritten task-2855
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2855;

pub fn little_endian_registers_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
            .collect()
    };
    samples.dedup();
    samples
}
