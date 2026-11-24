// rust-track: handwritten task-554
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 554;

pub fn puzzle_cells(row: &str) -> (usize, usize) {
    (
        row.chars().filter(|&c| c != '·').count(),
        row.chars().filter(|&c| c == '·').count(),
    )
}
