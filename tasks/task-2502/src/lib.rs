#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn abi_unsigned_widening_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    let _ = samples;
    0
}
