// rust-track: handwritten task-2386
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2386;

pub fn rolling_parity_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
