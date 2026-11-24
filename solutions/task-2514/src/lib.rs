// rust-track: handwritten task-2514
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2514;

pub fn abi_flags_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x != 0)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
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
