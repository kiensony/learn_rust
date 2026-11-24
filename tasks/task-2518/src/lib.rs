#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn abi_flags_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            extern "C" fn transform(x: u8) -> u32 {
                u32::from(x != 0)
            }
            let callback: extern "C" fn(u8) -> u32 = transform;
            input.iter().map(|&x| callback(x)).collect()
        };
        samples.reverse();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
