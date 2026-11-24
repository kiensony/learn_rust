// rust-track: handwritten task-2359
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2359;

pub fn single_pass_prefix_maximum_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut m = 0;
        input
            .iter()
            .map(|&x| {
                m = m.max(x);
                u32::from(m)
            })
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
