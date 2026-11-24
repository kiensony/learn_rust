// rust-track: handwritten task-2929
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2929;

pub fn failure_threshold_longest_run(input: &[u8]) -> u64 {
    let samples = observe_failure_threshold(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_failure_threshold(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
        .collect()
}
