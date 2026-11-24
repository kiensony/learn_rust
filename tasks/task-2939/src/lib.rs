#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn stability_band_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            input
                .chunks_exact(2)
                .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
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
