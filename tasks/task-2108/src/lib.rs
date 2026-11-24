#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn sorted_keys_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    let _ = samples;
    Vec::new()
}
