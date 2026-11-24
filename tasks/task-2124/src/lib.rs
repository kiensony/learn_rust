#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn queue_rotation_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_back() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
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
