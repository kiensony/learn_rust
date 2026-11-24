#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn latency_increases_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .rchunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
