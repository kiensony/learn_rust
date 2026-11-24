#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn abi_flags_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            extern "C" fn transform(x: u8) -> u32 {
                u32::from(x != 0)
            }
            let callback: extern "C" fn(u8) -> u32 = transform;
            input.iter().map(|&x| callback(x)).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
