// rust-track: handwritten task-2989
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2989;

pub fn service_wait_longest_run(input: &[u8]) -> u64 {
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
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
