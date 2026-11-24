// rust-track: handwritten task-2139
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2139;

pub fn histogram_bins_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
