// rust-track: handwritten task-2239
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2239;

pub fn zero_copy_lines_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
