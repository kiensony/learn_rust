// rust-track: handwritten task-1579
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1579;

pub fn feature_packing_longest_run(input: &[u8]) -> u64 {
    let samples = observe_feature_packing(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_feature_packing(input: &[u8]) -> Vec<u32> {
    let stride = if cfg!(feature = "wide") { 8u32 } else { 1u32 };
    input.iter().map(|&b| u32::from(b) * stride).collect()
}
