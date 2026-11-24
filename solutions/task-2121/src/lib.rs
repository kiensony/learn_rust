// rust-track: handwritten task-2121
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2121;

pub fn queue_rotation_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples
}
