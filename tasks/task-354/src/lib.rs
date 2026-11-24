#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

struct Visit<'a>(&'a std::cell::Cell<u8>);
impl Drop for Visit<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

pub fn closed_visits(visits: u8) -> u8 {
    let _ = visits;
    0
}
