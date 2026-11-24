// rust-track: handwritten task-2242
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2242;

pub fn borrowed_windows_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .as_bytes()
            .windows(2)
            .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
