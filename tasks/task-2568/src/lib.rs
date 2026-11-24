#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn odd_element_proof_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut owned = input.to_vec();
            let mut out = vec![];
            for cell in owned.iter_mut().skip(1).step_by(2) {
                *cell = cell.wrapping_add(1);
                out.push(u32::from(*cell));
            }
            out
        };
        samples.reverse();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
