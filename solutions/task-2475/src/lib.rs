// rust-track: handwritten task-2475
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2475;

pub fn strided_raw_reads_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let base = input.as_ptr();
        (0..input.len())
            .step_by(2)
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
