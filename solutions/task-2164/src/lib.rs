// rust-track: handwritten task-2164
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2164;

pub fn sorted_insertion_moves_peaks(input: &[u8]) -> Vec<u32> {
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
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
