// rust-track: handwritten task-2123
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2123;

pub fn queue_rotation_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
