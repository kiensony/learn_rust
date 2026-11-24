// rust-track: handwritten task-1763
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1763;

pub fn generated_offset_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
