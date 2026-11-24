// rust-track: handwritten task-1897
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1897;

pub fn weak_read_handles_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        use std::sync::Arc;
        input
            .iter()
            .map(|&x| {
                let mut data = Arc::new(x);
                let _owners: Vec<_> = (0..x % 4).map(|_| Arc::downgrade(&data)).collect();
                Arc::weak_count(&data) as u32
            })
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
