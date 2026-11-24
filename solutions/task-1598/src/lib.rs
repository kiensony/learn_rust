// rust-track: handwritten task-1598
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1598;

pub fn feature_packing_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 8u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
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
