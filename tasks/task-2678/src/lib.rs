#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn lending_checksum_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
