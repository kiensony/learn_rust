// rust-track: handwritten task-2494
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2494;

pub fn interior_raw_reads_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_interior_raw_reads(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_interior_raw_reads(input: &[u8]) -> Vec<u32> {
    let base = input.as_ptr();
    (1..input.len().saturating_sub(1))
        .map(|i| {
            let ptr = base.wrapping_add(i);
            debug_assert_eq!(ptr, &input[i] as *const u8);
            u32::from(input[i])
        })
        .collect()
}
