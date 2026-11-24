// rust-track: handwritten task-2536
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2536;

pub fn abi_endian_swap_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(u16::from(x).swap_bytes())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
