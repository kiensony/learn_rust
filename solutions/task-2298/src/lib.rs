// rust-track: handwritten task-2298
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2298;

pub fn exclusive_hotspots_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c == 1)
            .map(|(k, _)| u32::from(k))
            .collect()
    };
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
