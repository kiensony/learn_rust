// rust-track: handwritten task-2744
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2744;

pub fn cli_stems_peaks(input: &str) -> Vec<u32> {
    let samples = observe_cli_stems(input);
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

fn observe_cli_stems(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(std::path::Path::new)
        .filter_map(|p| p.file_stem())
        .map(|n| n.len() as u32)
        .collect()
}
