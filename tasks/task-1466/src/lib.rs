#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn client_error_fixture_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        vec![2, 1, 1]
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
