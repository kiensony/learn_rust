#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn repeated_keys_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.dedup();
    samples
}
