#[unsafe(no_mangle)]
pub extern "C" fn sample(raw: u16) -> i32 {
    core_sensor::unpack(raw).map_or(-1, i32::from)
}
