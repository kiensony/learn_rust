#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 69;

pub fn busy(initial: bool, reset: bool) -> bool {
    let busy = initial;
    if reset {
        busy = false;
    }
    busy
}
