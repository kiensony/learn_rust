#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn stride_proof_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut owned = input.to_vec();
            let mut out = vec![];
            for cell in owned.iter_mut().skip(1).step_by(3) {
                *cell = cell.wrapping_add(1);
                out.push(u32::from(*cell));
            }
            out
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
