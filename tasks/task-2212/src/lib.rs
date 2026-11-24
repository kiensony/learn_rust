#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_offsets_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().map(u64::from).sum()
}
