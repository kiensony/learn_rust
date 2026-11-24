#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn open_count(places: &[bool]) -> usize {
    let mut count = 0;
    for &open in places {
        if !open {
            break;
        }
        count += 1;
    }
    count
}
