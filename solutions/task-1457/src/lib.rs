// rust-track: handwritten task-1457
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1457;

pub fn http_status_fixture_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_http_status_fixture(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_http_status_fixture(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
        .filter_map(|s| s.parse::<u32>().ok())
        .filter(|x| (100..600).contains(x))
        .map(|x| x / 100)
        .collect()
}
