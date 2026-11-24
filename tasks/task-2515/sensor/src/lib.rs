#[repr(C)]
pub struct Packet {
    reading: i16,
    scale: u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn track_sensor_value(packet: Packet) -> i32 {
    i32::from(packet.reading) * i32::from(packet.scale)
}
