#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn locked_batch_totals_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let journal = std::sync::Mutex::new(Vec::new());
            for s in input.chunks(2) {
                let item = s.iter().copied().map(u32::from).sum();
                journal.lock().unwrap().push(item);
            }
            journal.into_inner().unwrap()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.dedup();
    samples
}
