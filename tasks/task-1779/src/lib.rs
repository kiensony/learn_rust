#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn build_mask_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
            input.iter().map(|&x| u32::from(x) & units).collect()
        };
        samples.sort_unstable();
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
