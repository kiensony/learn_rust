#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn big_endian_registers_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks_exact(2)
                .map(|c| u32::from(u16::from_be_bytes([c[0], c[1]])))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
