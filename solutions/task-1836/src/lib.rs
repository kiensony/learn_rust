// rust-track: handwritten task-1836
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1836;

pub fn shard_occupancy_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let chunks: Vec<_> = input.chunks(5).collect();
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
