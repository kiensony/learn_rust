#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn build_multiplier_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
            input.iter().map(|&x| u32::from(x) * units).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
