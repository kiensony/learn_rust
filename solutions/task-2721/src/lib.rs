// rust-track: handwritten task-2721
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2721;

pub fn cli_path_depth_trace(input: &str) -> Vec<u32> {
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
    samples
}
