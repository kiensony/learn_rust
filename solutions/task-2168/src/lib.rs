// rust-track: handwritten task-2168
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2168;

pub fn sorted_insertion_moves_steps(input: &[u8]) -> Vec<u32> {
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
