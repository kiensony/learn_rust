// rust-track: handwritten task-2486
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2486;

pub fn trailing_raw_reads_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
