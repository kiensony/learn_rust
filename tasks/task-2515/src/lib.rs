pub const TRACK_PROOF: u32 = 0;

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
    let packet = Packet {
        reading: i16::from(reading as u8),
        scale,
    };
    track_sensor_value(packet)
}
