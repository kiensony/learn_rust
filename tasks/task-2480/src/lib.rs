#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn strided_raw_reads_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let base = input.as_ptr();
        (0..input.len())
            .step_by(2)
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[0])
            })
            .collect()
    };
    samples.into_iter().rev().collect()
}
