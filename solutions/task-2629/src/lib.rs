// rust-track: handwritten task-2629
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2629;

pub fn embedded_parity_longest_run(input: &[u8]) -> u64 {
    let samples = observe_embedded_parity(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_embedded_parity(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(track_device::convert(x)))
        .collect()
}
