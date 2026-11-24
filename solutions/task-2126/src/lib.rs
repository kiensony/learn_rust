// rust-track: handwritten task-2126
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2126;

pub fn queue_rotation_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
