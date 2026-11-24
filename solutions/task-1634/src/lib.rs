// rust-track: handwritten task-1634
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1634;

pub fn clamp_property_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_clamp_property(input);
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

fn observe_clamp_property(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(x.clamp(4, 16) != x))
        .collect()
}

#[cfg(test)]
mod property_tests {
    #[test]
    fn exhaustive_byte_domain_agrees_with_widened_model() {
        for byte in u8::MIN..=u8::MAX {
            let wide = u16::from(byte);
            let property = !(4..=16).contains(&wide);
            assert_eq!(
                super::observe_clamp_property(&[byte]),
                [u32::from(property)],
                "first counterexample: {byte}"
            );
        }
    }
}
