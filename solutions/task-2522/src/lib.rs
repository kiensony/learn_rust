// rust-track: handwritten task-2522
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2522;

pub fn abi_signed_decode_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from((x as i8).unsigned_abs())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
