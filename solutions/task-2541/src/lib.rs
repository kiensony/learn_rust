// rust-track: handwritten task-2541
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2541;

pub fn abi_status_mask_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x & 0x7f)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples
}
