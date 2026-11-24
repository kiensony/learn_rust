#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn interior_raw_reads_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let base = input.as_ptr();
            (1..input.len().saturating_sub(1))
                .map(|i| {
                    let ptr = base.wrapping_add(i);
                    debug_assert_eq!(ptr, &input[i] as *const u8);
                    u32::from(input[i])
                })
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
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
