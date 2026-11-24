#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parity_property_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        vec![2, 1, 1]
    };
    samples.into_iter().map(u64::from).sum()
}
