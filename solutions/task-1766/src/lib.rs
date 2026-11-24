// rust-track: handwritten task-1766
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1766;

pub fn generated_offset_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
