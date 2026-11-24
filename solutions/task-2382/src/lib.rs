// rust-track: handwritten task-2382
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2382;

pub fn rolling_parity_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut p = 0;
        input
            .iter()
            .map(|&x| {
                p ^= x;
                u32::from(p)
            })
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
