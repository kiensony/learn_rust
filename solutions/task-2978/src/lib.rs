// rust-track: handwritten task-2978
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2978;

pub fn queue_depth_steps(input: &[u8]) -> Vec<u32> {
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
