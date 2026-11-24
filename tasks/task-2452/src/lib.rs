#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn forward_raw_reads_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let base = input.as_ptr();
        (0..input.len())
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[i])
            })
            .collect()
    };
    let _ = samples;
    0
}
