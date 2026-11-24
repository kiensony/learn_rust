// rust-track: handwritten task-929
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 929;

pub fn provide(value: Option<String>, fallback: impl FnOnce() -> String) -> String {
    value.unwrap_or_else(fallback)
}
