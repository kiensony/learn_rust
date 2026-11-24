// rust-track: handwritten task-2167
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2167;

pub fn sorted_insertion_moves_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_sorted_insertion_moves(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_sorted_insertion_moves(input: &[u8]) -> Vec<u32> {
    let mut sorted = Vec::new();
    input
        .iter()
        .map(|&x| {
            let i = sorted.partition_point(|&y| y <= x);
            let moves = sorted.len() - i;
            sorted.insert(i, x);
            moves as u32
        })
        .collect()
}
