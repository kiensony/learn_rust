// rust-track: handwritten task-2106
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2106;

pub fn sorted_keys_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
