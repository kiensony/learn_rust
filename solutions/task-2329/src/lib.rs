// rust-track: handwritten task-2329
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2329;

pub fn batch_medians_longest_run(input: &[u8]) -> u64 {
    let samples = observe_batch_medians(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_batch_medians(input: &[u8]) -> Vec<u32> {
    input
        .chunks(3)
        .map(|c| {
            let mut v = c.to_vec();
            v.sort_unstable();
            u32::from(v[(v.len() - 1) / 2])
        })
        .collect()
}
