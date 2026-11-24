// rust-track: handwritten task-2966
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2966;

pub fn loss_accounting_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::new();
        let mut obs = vec![];
        for (tick, &x) in input.iter().enumerate() {
            if x == 0 {
                if let Some((age, value)) = q.pop_front() {
                    let _ = (age, value);
                }
            } else if q.len() == 3 {
                obs.push(u32::from(x));
            } else {
                q.push_back((tick, x));
            }
        }
        obs
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
