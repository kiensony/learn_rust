// rust-track: handwritten task-2387
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2387;

pub fn rolling_parity_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut p = 0;
        input
            .iter()
            .map(|&x| {
                p ^= x;
                u32::from(p)
            })
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
