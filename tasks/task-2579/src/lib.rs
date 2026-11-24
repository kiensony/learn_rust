#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn triplet_proof_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut owned = input.to_vec();
            let mut out = vec![];
            for cell in owned.iter_mut().skip(0).step_by(3) {
                *cell = cell.wrapping_add(1);
                out.push(u32::from(*cell));
            }
            out
        };
        samples.truncate(samples.len().saturating_sub(1));
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
