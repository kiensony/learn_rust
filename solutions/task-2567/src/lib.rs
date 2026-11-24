// rust-track: handwritten task-2567
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2567;

pub fn odd_element_proof_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_odd_element_proof(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_odd_element_proof(input: &[u8]) -> Vec<u32> {
    let mut owned = input.to_vec();
    let mut out = vec![];
    for cell in owned.iter_mut().skip(1).step_by(2) {
        *cell = cell.wrapping_add(1);
        out.push(u32::from(*cell));
    }
    out
}
