// rust-track: handwritten task-2532
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2532;

pub fn abi_endian_swap_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(u16::from(x).swap_bytes())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
