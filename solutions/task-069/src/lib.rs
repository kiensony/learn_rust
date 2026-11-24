// rust-track: handwritten task-069
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 69;

pub fn busy(initial: bool, reset: bool) -> bool {
    let mut busy = initial;
    if reset {
        busy = false;
    }
    busy
}
