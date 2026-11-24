#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn build_mask_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) & units).collect()
    };
    let _ = samples;
    0
}
