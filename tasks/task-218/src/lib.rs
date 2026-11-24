#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 218;

pub fn stop(counter: i16) -> u16 {
    while true {
        break counter.max(0) as u16;
    }
}
