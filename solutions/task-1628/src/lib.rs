// rust-track: handwritten task-1628
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1628;

pub fn round_trip_property_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
