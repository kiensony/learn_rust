#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn first_occurrences_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut seen = std::collections::HashSet::new();
        input
            .iter()
            .filter(|&&x| seen.insert(x))
            .map(|&x| u32::from(x))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
