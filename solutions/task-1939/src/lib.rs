// rust-track: handwritten task-1939
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1939;

pub fn locked_batch_totals_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(2) {
            let item = s.iter().copied().map(u32::from).sum();
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
