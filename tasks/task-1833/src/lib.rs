#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shard_occupancy_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let chunks: Vec<_> = input.chunks_exact(5).collect();
        let (a, b) = chunks.split_at(chunks.len() / 2);
        std::thread::scope(|scope| {
            let work = |part: &[&[u8]]| {
                part.iter()
                    .map(|s| s.iter().filter(|&&x| x != 0).count() as u32)
                    .collect::<Vec<u32>>()
            };
            let first = scope.spawn(move || work(a));
            let second = work(b);
            let mut all = first.join().unwrap();
            all.extend(second);
            all
        })
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
