// rust-track: handwritten task-2506
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2506;

pub fn abi_unsigned_widening_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
