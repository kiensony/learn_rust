// rust-track: handwritten task-2872
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2872;

pub fn flag_register_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0] & c[1]))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
