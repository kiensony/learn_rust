// rust-track: handwritten task-2161
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2161;

pub fn sorted_insertion_moves_trace(input: &[u8]) -> Vec<u32> {
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
    samples
}
