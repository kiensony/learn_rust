#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packet_parity_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.into_iter().map(u64::from).sum()
}
