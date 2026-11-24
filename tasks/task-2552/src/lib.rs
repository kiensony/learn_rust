#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn split_proof_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(0).step_by(2) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    let _ = samples;
    0
}
