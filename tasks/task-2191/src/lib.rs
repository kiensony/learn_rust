#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn duplicate_insertion_cost_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut seen = std::collections::BTreeSet::new();
        input
            .iter()
            .map(|&x| {
                seen.insert(x);
                seen.len() as u32
            })
            .collect()
    };
    let _ = samples;
    Vec::new()
}
