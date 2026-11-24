#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 416;

fn advance(cursor: &mut u8) {
    *cursor = cursor.saturating_add(1);
}
pub fn next_cursor(cursor: u8) -> u8 {
    advance(&mut cursor);
    cursor
}
