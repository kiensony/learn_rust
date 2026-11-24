// rust-track: handwritten task-094
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 94;

pub fn loose_tiles_8(start: u16, end: u16) -> u32 {
    loose_tiles_8_impl(start, end)
}

fn loose_tiles_8_impl(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value) % 8
}
