// rust-track: handwritten task-1847
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1847;

pub fn shard_maxima_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        let chunks: Vec<_> = input.chunks(3).collect();
        let (a, b) = chunks.split_at(chunks.len() / 2);
        std::thread::scope(|scope| {
            let work = |part: &[&[u8]]| {
                part.iter()
                    .map(|s| s.iter().copied().max().map(u32::from).unwrap_or(0))
                    .collect::<Vec<u32>>()
            };
            let first = scope.spawn(move || work(a));
            let second = work(b);
            let mut all = first.join().unwrap();
            all.extend(second);
            all
        })
    };
    let mut runs: Vec<(u32, usize)> = Vec::new();
    for value in observations {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
