// rust-track: handwritten task-2729
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2729;

pub fn cli_path_depth_longest_run(input: &str) -> u64 {
    let samples = observe_cli_path_depth(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_cli_path_depth(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(std::path::Path::new)
        .map(|p| {
            p.components()
                .filter(|c| matches!(c, std::path::Component::Normal(_)))
                .count() as u32
        })
        .collect()
}
