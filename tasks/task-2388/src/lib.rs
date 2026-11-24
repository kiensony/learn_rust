#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn rolling_parity_steps(input: &[u8]) -> Vec<u32> {
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
    let _ = samples;
    Vec::new()
}
