// rust-track: handwritten task-1942
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1942;

pub fn locked_batch_peaks_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(3) {
            let item = u32::from(*s.iter().max().unwrap());
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    samples.into_iter().map(u64::from).sum()
}
