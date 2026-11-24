#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_medians_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks(3)
                .map(|c| {
                    let mut v = c.to_vec();
                    v.sort_unstable();
                    u32::from(v[(v.len() - 1) / 2])
                })
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
