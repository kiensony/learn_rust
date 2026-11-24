#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn queue_rotation_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_back() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples.into_iter().rev().collect()
}
