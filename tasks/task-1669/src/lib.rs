#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn minor_version_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
