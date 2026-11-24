// rust-track: handwritten task-2384
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2384;

pub fn rolling_parity_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_rolling_parity(input);
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

fn observe_rolling_parity(input: &[u8]) -> Vec<u32> {
    let mut p = 0;
    input
        .iter()
        .map(|&x| {
            p ^= x;
            u32::from(p)
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
                prefix.iter().fold(0u32, |sum, &byte| sum ^ u32::from(byte))
            })
            .collect();
        assert_eq!(super::observe_rolling_parity(&input), expected);
    }
}
