unsafe extern "C" {
    safe fn meter_value() -> u32;
}
fn main() {
    assert_eq!(meter_value(), 41);
}
