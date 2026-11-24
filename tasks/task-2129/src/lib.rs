#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn queue_rotation_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut q = std::collections::VecDeque::from(input.to_vec());
            if let Some(x) = q.pop_front() {
                q.push_back(x)
            }
            q.into_iter().map(u32::from).collect()
        };
        samples.sort_unstable();
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
