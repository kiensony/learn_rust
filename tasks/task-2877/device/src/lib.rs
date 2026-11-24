#![no_std]
pub fn sample(raw: u16) -> Option<u16> {
    core_sensor::unpack(raw)
}
