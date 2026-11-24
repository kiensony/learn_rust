// rust-track: handwritten task-2122
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2122;

pub fn queue_rotation_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
