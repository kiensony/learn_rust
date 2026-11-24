#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn hot_code_addresses_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            let mut samples = std::collections::BTreeMap::new();
            for &addr in input {
                *samples.entry(addr).or_insert(0u32) += 1;
            }
            samples
                .into_iter()
                .filter(|(_, c)| *c >= 2)
                .map(|(k, _)| u32::from(k))
                .collect()
        };
        observations.iter().collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
