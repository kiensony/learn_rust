// rust-track: handwritten task-2990
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2990;

pub fn service_wait_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::new();
        let mut obs = vec![];
        for (tick, &x) in input.iter().enumerate() {
            if x == 0 {
                if let Some((age, value)) = q.pop_front() {
                    let _ = (age, value);
                    obs.push((tick - age) as u32);
                }
            } else if q.len() == 5 {
            } else {
                q.push_back((tick, x));
            }
        }
        obs
    };
    samples.into_iter().rev().collect()
}
