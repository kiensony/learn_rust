// rust-track: handwritten task-1497
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1497;

pub fn client_error_fixture_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..600).contains(x))
            .filter(|x| (400..500).contains(x))
            .collect()
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
