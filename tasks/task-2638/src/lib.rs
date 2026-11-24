#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_saturation_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
