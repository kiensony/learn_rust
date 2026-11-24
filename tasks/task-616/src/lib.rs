#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

struct Meter {
    reading: u8,
}
impl Meter {
    fn advance(&mut self, by: u8) {
        self.reading = self.reading.saturating_add(by);
    }
}

pub fn after(start: u8, steps: &[u8]) -> u8 {
    start
}
