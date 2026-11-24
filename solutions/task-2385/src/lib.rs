// rust-track: handwritten task-2385
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2385;

pub fn rolling_parity_changes(input: &[u8]) -> Vec<u32> {
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
    samples.dedup();
    samples
}
