#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cumulative_occupancy_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut n = 0;
            input
                .iter()
                .map(|&x| {
                    n += u32::from(x != 0);
                    n
                })
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
