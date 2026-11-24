// rust-track: handwritten task-2558
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2558;

pub fn split_proof_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(0).step_by(2) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
