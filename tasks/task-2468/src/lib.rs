#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn reverse_raw_reads_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let base = input.as_ptr();
        (0..input.len())
            .rev()
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[0])
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
