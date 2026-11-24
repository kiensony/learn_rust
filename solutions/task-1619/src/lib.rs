// rust-track: handwritten task-1619
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1619;

pub fn saturation_property_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.saturating_add(1) == x))
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
