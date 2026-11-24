// rust-track: handwritten task-1577
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1577;

pub fn native_archive(component: &str) -> String {
    format!("track_{}", component.replace('-', "_"))
}
