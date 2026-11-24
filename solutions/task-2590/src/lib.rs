// rust-track: handwritten task-2590
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2590;

pub fn stride_proof_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(1).step_by(3) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    samples.into_iter().rev().collect()
}
