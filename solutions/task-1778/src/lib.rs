// rust-track: handwritten task-1778
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1778;

pub fn build_mask_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) & units).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
