#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn client_error_fixture_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split_ascii_whitespace()
                .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
                .filter_map(|s| s.parse::<u32>().ok())
                .filter(|x| (100..600).contains(x))
                .filter(|x| (400..500).contains(x))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
