// rust-track: handwritten task-236
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 236;

pub fn open_count(places: &[bool]) -> usize {
    let mut count = 0;
    for &open in places {
        if !open {
            continue;
        }
        count += 1;
    }
    count
}
