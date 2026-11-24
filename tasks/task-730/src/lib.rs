#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn port(text: &str) -> Option<u16> {
    text.parse::<u16>().ok().filter(|&p| p != 0)
}
