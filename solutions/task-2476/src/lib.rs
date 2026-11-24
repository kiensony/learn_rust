// rust-track: handwritten task-2476
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2476;

pub fn strided_raw_reads_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
