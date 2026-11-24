// rust-track: handwritten task-2626
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2626;

pub fn embedded_parity_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
