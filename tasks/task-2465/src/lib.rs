#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn reverse_raw_reads_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
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
    samples.dedup();
    samples
}
