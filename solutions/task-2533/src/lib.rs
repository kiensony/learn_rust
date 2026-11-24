// rust-track: handwritten task-2533
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2533;

pub fn abi_endian_swap_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(u16::from(x).swap_bytes())
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
