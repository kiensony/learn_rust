#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_path_depth_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
