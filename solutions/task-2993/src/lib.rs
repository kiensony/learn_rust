// rust-track: handwritten task-2993
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2993;

pub fn drain_order_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
