// rust-track: handwritten task-2127
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2127;

pub fn queue_rotation_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
