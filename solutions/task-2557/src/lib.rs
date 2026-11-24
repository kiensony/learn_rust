// rust-track: handwritten task-2557
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2557;

pub fn split_proof_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_split_proof(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_split_proof(input: &[u8]) -> Vec<u32> {
    let mut owned = input.to_vec();
    let mut out = vec![];
    for cell in owned.iter_mut().skip(0).step_by(2) {
        *cell = cell.wrapping_add(1);
        out.push(u32::from(*cell));
    }
    out
}
