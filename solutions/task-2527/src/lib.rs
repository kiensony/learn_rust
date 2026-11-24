// rust-track: handwritten task-2527
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2527;

pub fn abi_signed_decode_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from((x as i8).unsigned_abs())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
