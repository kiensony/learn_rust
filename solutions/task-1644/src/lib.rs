// rust-track: handwritten task-1644
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1644;

pub fn shift_property_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_shift_property(input);
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

fn observe_shift_property(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from((x >> 1) << 1 != x))
        .collect()
}

#[cfg(test)]
mod property_tests {
    #[test]
    fn exhaustive_byte_domain_agrees_with_widened_model() {
        for byte in u8::MIN..=u8::MAX {
            let wide = u16::from(byte);
            let property = wide % 2 == 1;
            assert_eq!(
                super::observe_shift_property(&[byte]),
                [u32::from(property)],
                "first counterexample: {byte}"
            );
        }
    }
}
