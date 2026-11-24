// rust-track: handwritten task-1616
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1616;

pub fn saturation_property_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.saturating_add(1) == x))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
