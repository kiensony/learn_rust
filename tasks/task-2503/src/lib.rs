#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2503;

pub type Callback = fn(u16) -> u16;
pub extern "C" fn identity(n: u16) -> u16 {
    n
}
pub fn default_callback() -> Callback {
    identity
}
pub fn invoke(cb: Callback, n: u16) -> u16 {
    cb(n)
}
