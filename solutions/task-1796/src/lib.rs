// rust-track: handwritten task-1796
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1796;

pub fn build_multiplier_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
