// rust-track: handwritten task-879
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 879;

mod wire {
    const WIDTHS: [usize; 3] = [1, 2, 4];
    pub(crate) fn width(code: u8) -> usize {
        WIDTHS[usize::from(code % 3)]
    }
}

pub fn code_size(code: u8) -> usize {
    wire::width(code)
}
