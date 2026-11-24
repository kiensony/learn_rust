#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn response_boundaries_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..500).contains(x))
            .enumerate()
            .filter(|(_, x)| *x < 400)
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.dedup();
    samples
}
