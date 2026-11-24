// rust-track: handwritten task-418
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 418;

pub fn cap(readings: &mut [u16], ceiling: u16) -> usize {
    let mut changed = 0;
    for reading in readings {
        if *reading > ceiling {
            *reading = ceiling;
            changed += 1;
        }
    }
    changed
}
