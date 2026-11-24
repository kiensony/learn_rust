// rust-track: handwritten task-2357
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2357;

pub fn single_pass_prefix_maximum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_single_pass_prefix_maximum(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_single_pass_prefix_maximum(input: &[u8]) -> Vec<u32> {
    let mut m = 0;
    input
        .iter()
        .map(|&x| {
            m = m.max(x);
            u32::from(m)
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
                u32::from(*prefix.iter().max().unwrap())
            })
            .collect();
        assert_eq!(super::observe_single_pass_prefix_maximum(&input), expected);
    }
}
