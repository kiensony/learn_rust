// rust-track: handwritten task-2322
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2322;

pub fn batch_medians_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks(3)
            .map(|c| {
                let mut v = c.to_vec();
                v.sort_unstable();
                u32::from(v[(v.len() - 1) / 2])
            })
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
