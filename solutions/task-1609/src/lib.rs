// rust-track: handwritten task-1609
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1609;

pub fn parity_property_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
