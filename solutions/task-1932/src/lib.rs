// rust-track: handwritten task-1932
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1932;

pub fn locked_batch_totals_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(2) {
            let item = s.iter().copied().map(u32::from).sum();
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    samples.into_iter().map(u64::from).sum()
}
