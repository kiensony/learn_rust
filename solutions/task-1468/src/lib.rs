// rust-track: handwritten task-1468
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1468;

pub fn client_error_fixture_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..600).contains(x))
            .filter(|x| (400..500).contains(x))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
