// rust-track: handwritten task-1309
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1309;

pub fn decimal_fields_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
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
