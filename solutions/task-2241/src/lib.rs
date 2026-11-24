// rust-track: handwritten task-2241
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2241;

pub fn borrowed_windows_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .as_bytes()
            .windows(2)
            .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
            .collect()
    };
    samples
}
