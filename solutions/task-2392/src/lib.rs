// rust-track: handwritten task-2392
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2392;

pub fn last_change_age_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
