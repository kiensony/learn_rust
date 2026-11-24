// rust-track: handwritten task-2229
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2229;

pub fn borrowed_scalars_longest_run(input: &str) -> u64 {
    let samples = observe_borrowed_scalars(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_borrowed_scalars(input: &str) -> Vec<u32> {
    input.char_indices().map(|(i, _)| i as u32).collect()
}
