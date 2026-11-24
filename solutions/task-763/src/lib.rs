// rust-track: handwritten task-763
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 763;

pub fn served(arrivals: &[u8], slots: usize) -> Vec<u8> {
    let mut queue: std::collections::VecDeque<_> = arrivals.iter().copied().collect();
    (0..slots.min(queue.len()))
        .filter_map(|_| queue.pop_front())
        .collect()
}
