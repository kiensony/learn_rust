// rust-track: handwritten task-2586
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2586;

pub fn stride_proof_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(1).step_by(3) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
