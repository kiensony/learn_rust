#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn locked_batch_totals_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let journal = std::sync::Mutex::new(Vec::new());
            for s in input.chunks(2) {
                let item = s.iter().copied().map(u32::from).sum();
                journal.lock().unwrap().push(item);
            }
            journal.into_inner().unwrap()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
