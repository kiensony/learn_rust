// rust-track: handwritten task-2109
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2109;

pub fn sorted_keys_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
