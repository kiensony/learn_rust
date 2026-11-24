#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn native_archive(component: &str) -> String {
    format!("track_{component}")
}
