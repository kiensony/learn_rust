// rust-track: handwritten task-1544
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1544;

pub fn packet_reversal_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_packet_reversal(input);
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

fn observe_packet_reversal(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| u32::from(wire::decode(x))).collect()
}
