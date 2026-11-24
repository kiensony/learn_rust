#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn last_change_age_trace(input: &[u8]) -> Vec<u32> {
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
    let _ = samples;
    Vec::new()
}
