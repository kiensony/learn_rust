#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn strided_raw_reads_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.dedup();
    samples
}
