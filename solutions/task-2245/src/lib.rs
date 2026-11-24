// rust-track: handwritten task-2245
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2245;

pub fn borrowed_windows_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .as_bytes()
            .windows(2)
            .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
            .collect()
    };
    samples.dedup();
    samples
}
