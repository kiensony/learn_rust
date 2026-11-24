#![forbid(unsafe_code)]

pub fn elapsed_seconds(ticks: u64) -> Option<u64> {
    ticks.checked_mul(units::SECONDS_PER_TICK)
}
