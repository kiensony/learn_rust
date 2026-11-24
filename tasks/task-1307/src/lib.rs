#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn decimal_fields_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .map(u32::from)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
