// rust-track: handwritten task-2646
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2646;

pub fn embedded_checksum_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
