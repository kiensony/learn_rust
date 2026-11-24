// rust-track: handwritten task-1607
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1607;

pub fn parity_property_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_parity_property(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_parity_property(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect()
}

#[cfg(test)]
mod property_tests {
    #[test]
    fn exhaustive_byte_domain_agrees_with_widened_model() {
        for byte in u8::MIN..=u8::MAX {
            let wide = u16::from(byte);
            let property = ((wide / 2) * 2 + u16::from(byte & 1)) != wide;
            assert_eq!(
                super::observe_parity_property(&[byte]),
                [u32::from(property)],
                "first counterexample: {byte}"
            );
        }
    }
}
