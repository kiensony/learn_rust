// rust-track: handwritten task-2622
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2622;

pub fn embedded_parity_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
