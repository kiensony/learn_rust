// rust-track: handwritten task-1107
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1107;

pub trait Source {
    fn text(&self) -> &str;
}
impl Source for String {
    fn text(&self) -> &str {
        self
    }
}
impl Source for &str {
    fn text(&self) -> &str {
        self
    }
}

pub fn prefix_before_final_mark_colon(source: &dyn Source) -> usize {
    prefix_before_final_mark_colon_impl(source)
}

fn prefix_before_final_mark_colon_impl(source: &dyn Source) -> usize {
    let text = source.text();
    (text.rsplit_once(':').map_or("", |(a, _)| a)).len()
}
