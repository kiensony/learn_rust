#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn interior_stride_proof_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(2).step_by(3) {
            *cell = cell.saturating_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
