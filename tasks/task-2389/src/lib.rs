#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn rolling_parity_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            let mut p = 0;
            input
                .iter()
                .map(|&x| {
                    p ^= x;
                    u32::from(p)
                })
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
