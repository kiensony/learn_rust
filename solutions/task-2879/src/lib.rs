// rust-track: handwritten task-2879
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2879;

pub fn flag_register_longest_run(input: &[u8]) -> u64 {
    let samples = observe_flag_register(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_flag_register(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|c| u32::from(c[0] & c[1]))
        .collect()
}
