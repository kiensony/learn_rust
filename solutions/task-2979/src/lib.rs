// rust-track: handwritten task-2979
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2979;

pub fn queue_depth_longest_run(input: &[u8]) -> u64 {
    let samples = observe_queue_depth(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_queue_depth(input: &[u8]) -> Vec<u32> {
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
}
