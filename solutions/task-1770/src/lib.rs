// rust-track: handwritten task-1770
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1770;

pub fn generated_offset_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples.into_iter().rev().collect()
}
