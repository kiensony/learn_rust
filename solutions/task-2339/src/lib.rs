// rust-track: handwritten task-2339
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2339;

pub fn batch_variation_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
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
