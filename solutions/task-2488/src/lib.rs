// rust-track: handwritten task-2488
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2488;

pub fn trailing_raw_reads_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let base = input.as_ptr();
        (input.len().saturating_sub(4)..input.len())
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[i])
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
