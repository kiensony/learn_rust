// rust-track: handwritten task-1729
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1729;

pub fn boolean_macro_longest_run(input: &[u8]) -> u64 {
    let samples = observe_boolean_macro(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

macro_rules! sample {
    ($x:expr) => {
        u32::from(($x) == 0)
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
        assert_eq!(value, 0u32);
    }
}

fn observe_boolean_macro(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| sample!(x)).collect()
}
