// rust-track: handwritten task-2967
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2967;

pub fn loss_accounting_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_loss_accounting(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_loss_accounting(input: &[u8]) -> Vec<u32> {
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
}
