// rust-track: handwritten task-2505
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2505;

pub fn abi_unsigned_widening_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.dedup();
    samples
}
