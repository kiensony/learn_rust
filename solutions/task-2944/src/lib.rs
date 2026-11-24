// rust-track: handwritten task-2944
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2944;

pub fn rollback_triggers_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_rollback_triggers(input);
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

fn observe_rollback_triggers(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(u8::from(u16::from(p[1]) > 2 * u16::from(p[0]))))
        .collect()
}
