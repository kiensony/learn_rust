// rust-track: handwritten task-1747
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1747;

pub fn saturating_macro_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let mut runs: Vec<(u32, usize)> = Vec::new();
    for value in observations {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
