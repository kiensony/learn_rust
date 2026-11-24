// rust-track: handwritten task-464
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 464;

pub fn prefix_before_final_mark_colon(fields: (&str, &str)) -> usize {
    prefix_before_final_mark_colon_impl(fields)
}

fn prefix_before_final_mark_colon_impl(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.rsplit_once(':').map_or("", |(a, _)| a)).len()
}
