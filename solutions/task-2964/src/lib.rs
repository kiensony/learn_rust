// rust-track: handwritten task-2964
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2964;

pub fn loss_accounting_peaks(input: &[u8]) -> Vec<u32> {
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
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
