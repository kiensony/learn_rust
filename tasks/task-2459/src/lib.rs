#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn forward_raw_reads_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let base = input.as_ptr();
        (0..input.len())
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[0])
            })
            .collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
