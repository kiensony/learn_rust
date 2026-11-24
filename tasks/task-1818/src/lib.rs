#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shard_maxima_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let chunks: Vec<_> = input.chunks_exact(3).collect();
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
