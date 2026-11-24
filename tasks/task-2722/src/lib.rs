#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_path_depth_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.into_iter().map(u64::from).sum()
}
