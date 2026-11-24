// rust-track: handwritten task-1762
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1762;

pub fn generated_offset_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
