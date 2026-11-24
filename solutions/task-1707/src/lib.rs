// rust-track: handwritten task-1707
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1707;

pub fn nibble_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_nibble_macro(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

macro_rules! sample {
    ($x:expr) => {
        ($x & 15) as u32
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
        assert_eq!(value, 13u32);
    }
}

fn observe_nibble_macro(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| sample!(x)).collect()
}
