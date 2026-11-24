#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn big_endian_registers_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .windows(2)
            .map(|c| u32::from(u16::from_be_bytes([c[0], c[1]])))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
