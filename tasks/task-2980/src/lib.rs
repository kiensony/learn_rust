#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn queue_depth_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
