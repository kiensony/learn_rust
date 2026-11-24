// rust-track: handwritten task-2515

pub const TRACK_PROOF: u32 = 0x52540000 | 2515;

extern crate sensor;
#[repr(C)]
struct Packet {
    reading: i16,
    scale: u8,
}
unsafe extern "C" {
    safe fn track_sensor_value(packet: Packet) -> i32;
}

pub fn scaled_reading(reading: i16, scale: u8) -> i32 {
    let packet = Packet { reading, scale };
    track_sensor_value(packet)
}
