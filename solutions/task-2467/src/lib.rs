// rust-track: handwritten task-2467
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2467;

pub fn reverse_raw_reads_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_reverse_raw_reads(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_reverse_raw_reads(input: &[u8]) -> Vec<u32> {
    let base = input.as_ptr();
    (0..input.len())
        .rev()
        .map(|i| {
            let ptr = base.wrapping_add(i);
            debug_assert_eq!(ptr, &input[i] as *const u8);
            u32::from(input[i])
        })
        .collect()
}
