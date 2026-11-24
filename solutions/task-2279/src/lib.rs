// rust-track: handwritten task-2279
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2279;

pub fn exclusive_hotspots_longest_run(input: &[u8]) -> u64 {
    let samples = observe_exclusive_hotspots(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_exclusive_hotspots(input: &[u8]) -> Vec<u32> {
    let mut samples = std::collections::BTreeMap::new();
    for &addr in input {
        *samples.entry(addr).or_insert(0u32) += 1;
    }
    samples
        .into_iter()
        .filter(|(_, c)| *c == 1)
        .map(|(k, _)| u32::from(k))
        .collect()
}
