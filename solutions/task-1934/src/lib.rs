// rust-track: handwritten task-1934
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1934;

pub fn locked_batch_totals_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_locked_batch_totals(input);
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

fn observe_locked_batch_totals(input: &[u8]) -> Vec<u32> {
    let journal = std::sync::Mutex::new(Vec::new());
    for s in input.chunks(2) {
        let item = s.iter().copied().map(u32::from).sum();
        journal.lock().unwrap().push(item);
    }
    journal.into_inner().unwrap()
}
