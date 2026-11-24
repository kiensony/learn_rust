#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn abi_signed_decode_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from((x as i8).unsigned_abs())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x & 0x3f)).collect()
    };
    samples.into_iter().rev().collect()
}
