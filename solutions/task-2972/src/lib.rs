// rust-track: handwritten task-2972
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2972;

pub fn queue_depth_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::new();
        let mut obs = vec![];
        for (tick, &x) in input.iter().enumerate() {
            if x == 0 {
                if let Some((age, value)) = q.pop_front() {
                    let _ = (age, value);
                }
            } else if q.len() == 4 {
            } else {
                q.push_back((tick, x));
            }
            obs.push(q.len() as u32);
        }
        obs
    };
    samples.into_iter().map(u64::from).sum()
}
