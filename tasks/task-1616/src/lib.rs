#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturation_property_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
