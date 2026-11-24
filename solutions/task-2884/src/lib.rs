// rust-track: handwritten task-2884
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2884;

pub fn packed_pin_state_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_packed_pin_state(input);
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

fn observe_packed_pin_state(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
        .collect()
}
