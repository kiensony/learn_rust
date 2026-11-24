#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn abi_flags_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            extern "C" fn transform(x: u8) -> u32 {
                u32::from(x != 0)
            }
            let callback: extern "C" fn(u8) -> u32 = transform;
            input.iter().map(|&x| callback(x)).collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().map(u64::from).sum()
}
