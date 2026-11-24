// rust-track: handwritten task-2148
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2148;

pub fn queue_rotation_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
        let mut q = std::collections::VecDeque::from(input.to_vec());
        if let Some(x) = q.pop_front() {
            q.push_back(x)
        }
        q.into_iter().map(u32::from).collect()
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
