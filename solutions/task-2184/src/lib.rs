// rust-track: handwritten task-2184
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2184;

pub fn early_exit_probes_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_early_exit_probes(input);
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

fn observe_early_exit_probes(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .enumerate()
        .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
        .collect()
}
