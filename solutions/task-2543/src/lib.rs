// rust-track: handwritten task-2543
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2543;

pub fn abi_status_mask_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x & 0x7f)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
