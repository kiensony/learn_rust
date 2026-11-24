// rust-track: handwritten task-1744
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1744;

pub fn rotate_macro_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_rotate_macro(input);
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

macro_rules! sample {
    ($x:expr) => {
        u32::from(($x).rotate_left(2))
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
        assert_eq!(value, 52u32);
    }
}

fn observe_rotate_macro(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| sample!(x)).collect()
}
