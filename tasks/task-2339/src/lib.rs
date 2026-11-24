#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_variation_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            input
                .chunks(4)
                .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
                .collect()
        };
        observations.iter().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
