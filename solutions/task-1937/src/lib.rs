// rust-track: handwritten task-1937
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1937;

pub fn locked_batch_totals_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(2) {
            let item = s.iter().copied().map(u32::from).sum();
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
