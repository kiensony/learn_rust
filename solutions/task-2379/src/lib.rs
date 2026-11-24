// rust-track: handwritten task-2379
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2379;

pub fn cumulative_occupancy_longest_run(input: &[u8]) -> u64 {
    let samples = observe_cumulative_occupancy(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_cumulative_occupancy(input: &[u8]) -> Vec<u32> {
    let mut n = 0;
    input
        .iter()
        .map(|&x| {
            n += u32::from(x != 0);
            n
        })
        .collect()
}

#[cfg(test)]
mod oracle_tests {
    #[test]
    fn repeated_prefix_scan_checks_incremental_state() {
        let input: Vec<u8> = (0..128u16).map(|n| ((n / 3) * 29 % 41) as u8).collect();
        let expected: Vec<_> = (1..=input.len())
            .map(|end| {
                let prefix = &input[..end];
                prefix.iter().filter(|&&byte| byte != 0).count() as u32
            })
            .collect();
        assert_eq!(super::observe_cumulative_occupancy(&input), expected);
    }
}
