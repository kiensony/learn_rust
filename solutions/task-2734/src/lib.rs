// rust-track: handwritten task-2734
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2734;

pub fn cli_rust_sources_peaks(input: &str) -> Vec<u32> {
    let samples = observe_cli_rust_sources(input);
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

fn observe_cli_rust_sources(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(std::path::Path::new)
        .enumerate()
        .filter(|(_, p)| p.extension().is_some_and(|e| e == "rs"))
        .map(|(i, _)| i as u32)
        .collect()
}
