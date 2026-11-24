#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn triplet_proof_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut owned = input.to_vec();
        let mut out = vec![];
        for cell in owned.iter_mut().skip(0).step_by(3) {
            *cell = cell.saturating_add(1);
            out.push(u32::from(*cell));
        }
        out
    };
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
