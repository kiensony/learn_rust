#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_fields_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .map(|x| u32::from(x >> 4))
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().map(u64::from).sum()
}
