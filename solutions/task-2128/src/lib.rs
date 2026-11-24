// rust-track: handwritten task-2128
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2128;

pub fn queue_rotation_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
