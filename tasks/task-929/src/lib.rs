#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn provide(value: Option<String>, fallback: impl FnOnce() -> String) -> String {
    let _probe = fallback();
    value.unwrap_or_else(fallback)
}
