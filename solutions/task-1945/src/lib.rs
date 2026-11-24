// rust-track: handwritten task-1945
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1945;

pub fn locked_batch_peaks_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(3) {
            let item = u32::from(*s.iter().max().unwrap());
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    samples.dedup();
    samples
}
