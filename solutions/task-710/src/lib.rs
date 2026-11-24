// rust-track: handwritten task-710
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 710;

pub fn prefix_before_final_mark_colon(record: Option<&str>) -> Option<u16> {
    prefix_before_final_mark_colon_impl(record)
}

fn prefix_before_final_mark_colon_impl(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.rsplit_once(':').map_or("", |(a, _)| a);
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
