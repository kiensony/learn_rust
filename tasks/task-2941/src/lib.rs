#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn rollback_triggers_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(u16::from(p[1]) > 2 * u16::from(p[0]))))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
