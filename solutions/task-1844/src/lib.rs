// rust-track: handwritten task-1844
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1844;

pub fn shard_checksum_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_shard_checksum(input);
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

fn observe_shard_checksum(input: &[u8]) -> Vec<u32> {
    let chunks: Vec<_> = input.chunks(6).collect();
    let (a, b) = chunks.split_at(chunks.len() / 2);
    std::thread::scope(|scope| {
        let work = |part: &[&[u8]]| {
            part.iter()
                .map(|s| s.iter().fold(0, |a, &x| a ^ u32::from(x)))
                .collect::<Vec<u32>>()
        };
        let first = scope.spawn(move || work(a));
        let second = work(b);
        let mut all = first.join().unwrap();
        all.extend(second);
        all
    })
}
