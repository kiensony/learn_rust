#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn client_error_fixture_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            input
                .split_ascii_whitespace()
                .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
                .filter_map(|s| s.parse::<u32>().ok())
                .filter(|x| (100..600).contains(x))
                .filter(|x| (400..500).contains(x))
                .collect()
        };
        observations.iter().collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
