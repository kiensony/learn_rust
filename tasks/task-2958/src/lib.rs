#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn journal_replay_steps(input: &[u8]) -> Vec<u32> {
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
    let _ = samples;
    Vec::new()
}
