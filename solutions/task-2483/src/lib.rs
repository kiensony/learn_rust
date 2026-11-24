// rust-track: handwritten task-2483
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2483;

pub fn trailing_raw_reads_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
