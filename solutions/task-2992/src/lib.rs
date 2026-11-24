// rust-track: handwritten task-2992
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2992;

pub fn drain_order_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut q = std::collections::VecDeque::new();
        let mut obs = vec![];
        for (tick, &x) in input.iter().enumerate() {
            if x == 0 {
                if let Some((age, value)) = q.pop_front() {
                    let _ = (age, value);
                }
            } else if q.len() == 6 {
            } else {
                q.push_back((tick, x));
            }
        }
        obs.extend(q.into_iter().map(|(_, x)| u32::from(x)));
        obs
    };
    samples.into_iter().map(u64::from).sum()
}
