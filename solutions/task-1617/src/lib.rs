// rust-track: handwritten task-1617
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1617;

pub fn saturation_property_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_saturation_property(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_saturation_property(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(x.saturating_add(1) == x))
        .collect()
}

#[cfg(test)]
mod property_tests {
    #[test]
    fn exhaustive_byte_domain_agrees_with_widened_model() {
        for byte in u8::MIN..=u8::MAX {
            let wide = u16::from(byte);
            let property = wide == u16::from(u8::MAX);
            assert_eq!(
                super::observe_saturation_property(&[byte]),
                [u32::from(property)],
                "first counterexample: {byte}"
            );
        }
    }
}
