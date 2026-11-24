// rust-track: handwritten task-2537
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2537;

pub fn abi_endian_swap_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(u16::from(x).swap_bytes())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
