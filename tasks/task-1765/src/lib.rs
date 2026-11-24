#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn generated_offset_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input
            .iter()
            .map(|&x| u32::from(x.saturating_sub(1)) + units)
            .collect()
    };
    samples.dedup();
    samples
}
