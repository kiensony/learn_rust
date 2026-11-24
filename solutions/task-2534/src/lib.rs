// rust-track: handwritten task-2534
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2534;

pub fn abi_endian_swap_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_abi_endian_swap(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_abi_endian_swap(input: &[u8]) -> Vec<u32> {
    extern "C" fn transform(x: u8) -> u32 {
        u32::from(u16::from(x).swap_bytes())
    }
    let callback: extern "C" fn(u8) -> u32 = transform;
    input.iter().map(|&x| callback(x)).collect()
}
