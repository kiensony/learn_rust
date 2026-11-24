// rust-track: handwritten task-2594
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2594;

pub fn interior_stride_proof_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_interior_stride_proof(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_interior_stride_proof(input: &[u8]) -> Vec<u32> {
    let mut owned = input.to_vec();
    let mut out = vec![];
    for cell in owned.iter_mut().skip(2).step_by(3) {
        *cell = cell.wrapping_add(1);
        out.push(u32::from(*cell));
    }
    out
}
