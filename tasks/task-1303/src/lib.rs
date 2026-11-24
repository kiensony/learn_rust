#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1303;

fn digits(port: &u8) -> usize {
    port.to_string().len()
}
pub fn port_width(port: Option<u8>) -> usize {
    port.map(digits).unwrap_or(0)
}
