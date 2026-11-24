// rust-track: handwritten task-2394
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2394;

pub fn last_change_age_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_last_change_age(input);
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

fn observe_last_change_age(input: &[u8]) -> Vec<u32> {
    let mut previous = None;
    let mut run = 0;
    input
        .iter()
        .map(|&x| {
            run = if previous == Some(x) { run + 1 } else { 1 };
            previous = Some(x);
            run
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
                prefix
                    .iter()
                    .rev()
                    .take_while(|&&byte| byte == prefix[prefix.len() - 1])
                    .count() as u32
            })
            .collect();
        assert_eq!(super::observe_last_change_age(&input), expected);
    }
}
