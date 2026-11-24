// rust-track: handwritten task-2502
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2502;

pub fn abi_unsigned_widening_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
