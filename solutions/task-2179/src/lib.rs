// rust-track: handwritten task-2179
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2179;

pub fn prefix_comparison_budget_longest_run(input: &[u8]) -> u64 {
    let samples = observe_prefix_comparison_budget(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_prefix_comparison_budget(input: &[u8]) -> Vec<u32> {
    (0..input.len()).map(|i| i as u32).collect()
}
