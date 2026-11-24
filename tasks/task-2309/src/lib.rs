#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn warmup_discard_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().skip(1).copied().map(u32::from).collect() };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
