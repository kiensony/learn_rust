// rust-track: handwritten task-2601
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2601;

pub fn embedded_checksum_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples
}
