// rust-track: handwritten task-1943
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1943;

pub fn locked_batch_peaks_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(3) {
            let item = u32::from(*s.iter().max().unwrap());
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
