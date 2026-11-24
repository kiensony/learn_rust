// rust-track: handwritten task-2592
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2592;

pub fn interior_stride_proof_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(2).step_by(3) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    samples.into_iter().map(u64::from).sum()
}
