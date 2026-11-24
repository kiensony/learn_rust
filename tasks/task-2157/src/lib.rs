#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn linear_search_probes_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
