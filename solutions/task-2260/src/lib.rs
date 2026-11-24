// rust-track: handwritten task-2260
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2260;

pub fn cpu_sample_counts_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(_, c)| c).collect()
    };
    samples.into_iter().rev().collect()
}
