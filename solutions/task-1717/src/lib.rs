// rust-track: handwritten task-1717
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1717;

pub fn saturating_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_saturating_macro(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

macro_rules! sample {
    ($x:expr) => {
        u32::from(($x).saturating_add(4))
    };
}

#[cfg(test)]
mod macro_tests {
    #[test]
    fn an_argument_is_evaluated_once() {
        let mut evaluations = 0;
        let byte = 13u8;
        let value = sample!({
            evaluations += 1;
            byte
        });
        assert_eq!(evaluations, 1);
        assert_eq!(value, 17u32);
    }
}

fn observe_saturating_macro(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| sample!(x)).collect()
}
