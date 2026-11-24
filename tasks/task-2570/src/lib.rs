#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn odd_element_proof_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(1).step_by(2) {
            *cell = cell.wrapping_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
    let _ = samples;
    Vec::new()
}
