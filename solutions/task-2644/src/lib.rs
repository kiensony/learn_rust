// rust-track: handwritten task-2644
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2644;

pub fn embedded_nibble_swap_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_embedded_nibble_swap(input);
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

fn observe_embedded_nibble_swap(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(track_device::convert(x)))
        .collect()
}
