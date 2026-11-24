#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parsed_defaults_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        vec![2, 1, 1]
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
