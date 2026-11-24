// rust-track: handwritten task-2638
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2638;

pub fn embedded_saturation_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
