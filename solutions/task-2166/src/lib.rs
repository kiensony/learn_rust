// rust-track: handwritten task-2166
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2166;

pub fn sorted_insertion_moves_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
