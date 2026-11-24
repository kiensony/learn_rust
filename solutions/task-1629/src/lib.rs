// rust-track: handwritten task-1629
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1629;

pub fn round_trip_property_longest_run(input: &[u8]) -> u64 {
    let samples = observe_round_trip_property(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_round_trip_property(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
        .collect()
}

#[cfg(test)]
mod property_tests {
    #[test]
    fn exhaustive_byte_domain_agrees_with_widened_model() {
        for byte in u8::MIN..=u8::MAX {
            let wide = u16::from(byte);
            let property = wide * 2 > 255;
            assert_eq!(
                super::observe_round_trip_property(&[byte]),
                [u32::from(property)],
                "first counterexample: {byte}"
            );
        }
    }
}
