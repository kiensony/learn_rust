#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn rolling_parity_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
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
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
