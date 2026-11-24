// rust-track: handwritten task-616
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 616;

struct Meter {
    reading: u8,
}
impl Meter {
    fn advance(&mut self, by: u8) {
        self.reading = self.reading.saturating_add(by);
    }
}

pub fn after(start: u8, steps: &[u8]) -> u8 {
    let mut meter = Meter { reading: start };
    for &step in steps {
        meter.advance(step);
    }
    meter.reading
}
