// rust-track: handwritten task-1117
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1117;

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

pub fn prefix_before_final_mark_slash(source: &dyn Source) -> usize {
    prefix_before_final_mark_slash_impl(source)
}

fn prefix_before_final_mark_slash_impl(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .len()
}
