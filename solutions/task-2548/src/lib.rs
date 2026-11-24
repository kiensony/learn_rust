// rust-track: handwritten task-2548
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2548;

pub fn abi_signed_decode_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from((x as i8).unsigned_abs())
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
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
