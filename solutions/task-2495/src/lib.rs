// rust-track: handwritten task-2495
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2495;

pub fn interior_raw_reads_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let base = input.as_ptr();
        (1..input.len().saturating_sub(1))
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[i])
            })
            .collect()
    };
    samples.dedup();
    samples
}
