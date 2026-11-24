// rust-track: handwritten task-1807
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1807;

pub fn shard_sums_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_shard_sums(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_shard_sums(input: &[u8]) -> Vec<u32> {
    let chunks: Vec<_> = input.chunks(2).collect();
    let (a, b) = chunks.split_at(chunks.len() / 2);
    std::thread::scope(|scope| {
        let work = |part: &[&[u8]]| {
            part.iter()
                .map(|s| s.iter().map(|&x| u32::from(x)).sum())
                .collect::<Vec<u32>>()
        };
        let first = scope.spawn(move || work(a));
        let second = work(b);
        let mut all = first.join().unwrap();
        all.extend(second);
        all
    })
}
