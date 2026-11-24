// rust-track: handwritten task-1456
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1456;

pub fn http_status_fixture_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..600).contains(x))
            .map(|x| x / 100)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
