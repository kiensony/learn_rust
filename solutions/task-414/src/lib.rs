// rust-track: handwritten task-414
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 414;

pub fn prefix_before_final_mark_colon(text: &mut String) -> () {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl(text: &mut String) -> () {
    let selected = (text.rsplit_once(':').map_or("", |(a, _)| a)).to_owned();
    text.clear();
    text.push_str(&selected);
}
