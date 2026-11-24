// rust-track: handwritten task-2395
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2395;

pub fn last_change_age_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
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
    samples.dedup();
    samples
}
