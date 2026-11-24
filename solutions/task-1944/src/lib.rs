// rust-track: handwritten task-1944
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1944;

pub fn locked_batch_peaks_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_locked_batch_peaks(input);
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

fn observe_locked_batch_peaks(input: &[u8]) -> Vec<u32> {
    let journal = std::sync::Mutex::new(Vec::new());
    for s in input.chunks(3) {
        let item = u32::from(*s.iter().max().unwrap());
        journal.lock().unwrap().push(item);
    }
    journal.into_inner().unwrap()
}
