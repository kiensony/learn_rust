// rust-track: handwritten task-104
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 104;

pub fn loose_tiles_10(requested: u16, available: u16) -> u32 {
    loose_tiles_10_impl(requested, available)
}

fn loose_tiles_10_impl(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value) % 10
}
