// rust-track: handwritten task-2945
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2945;

pub fn rollback_triggers_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(u16::from(p[1]) > 2 * u16::from(p[0]))))
            .collect()
    };
    samples.dedup();
    samples
}
