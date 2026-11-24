// rust-track: handwritten task-2297
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2297;

pub fn hot_code_addresses_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c >= 2)
            .map(|(k, _)| u32::from(k))
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
