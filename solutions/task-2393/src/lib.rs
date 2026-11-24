// rust-track: handwritten task-2393
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2393;

pub fn last_change_age_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut previous = None;
        let mut run = 0;
        input
            .iter()
            .map(|&x| {
                run = if previous == Some(x) { run + 1 } else { 1 };
                previous = Some(x);
                run
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
