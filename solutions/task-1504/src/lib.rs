// rust-track: handwritten task-1504
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1504;

pub fn packet_high_nibble_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
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
