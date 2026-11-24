// rust-track: handwritten task-2956
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2956;

pub fn journal_replay_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::new();
        let mut obs = vec![];
        for (tick, &x) in input.iter().enumerate() {
            if x == 0 {
                if let Some((age, value)) = q.pop_front() {
                    let _ = (age, value);
                    obs.push(u32::from(value));
                }
            } else if q.len() == 2 {
            } else {
                q.push_back((tick, x));
            }
        }
        obs.extend(q.into_iter().map(|(_, x)| u32::from(x)));
        obs
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
