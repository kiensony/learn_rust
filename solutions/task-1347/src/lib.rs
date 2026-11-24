// rust-track: handwritten task-1347
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1347;

pub fn nibble_fields_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
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
