// rust-track: handwritten task-354
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 354;

struct Visit<'a>(&'a std::cell::Cell<u8>);
impl Drop for Visit<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

pub fn closed_visits(visits: u8) -> u8 {
    let closed = std::cell::Cell::new(0);
    for _ in 0..visits {
        let _guard = Visit(&closed);
    }
    closed.get()
}
