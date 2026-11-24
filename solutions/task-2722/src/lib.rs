// rust-track: handwritten task-2722
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2722;

pub fn cli_path_depth_total(input: &str) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
