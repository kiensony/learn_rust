// rust-track: handwritten task-1479
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1479;

pub fn retry_fixture_longest_run(input: &str) -> u64 {
    let samples = observe_retry_fixture(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_retry_fixture(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
        .filter_map(|s| s.parse::<u32>().ok())
        .filter(|x| (100..600).contains(x))
        .filter(|x| *x >= 500)
        .collect()
}
