#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_medians_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    let _ = samples;
    None
}
