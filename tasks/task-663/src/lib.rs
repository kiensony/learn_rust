#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 663;

pub enum Shipment {
    Box(u8, u8),
    Letter(u8),
}
pub fn weight(s: Shipment) -> u16 {
    match s {
        Shipment::Box(a) => u16::from(a),
        Shipment::Letter(n) => u16::from(n),
    }
}
