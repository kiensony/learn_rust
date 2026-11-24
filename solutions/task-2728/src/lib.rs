// rust-track: handwritten task-2728
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2728;

pub fn cli_path_depth_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .map(|p| {
                p.components()
                    .filter(|c| matches!(c, std::path::Component::Normal(_)))
                    .count() as u32
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
