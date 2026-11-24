// rust-track: handwritten task-474
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 474;

pub fn prefix_before_final_mark_slash(fields: (&str, &str)) -> usize {
    prefix_before_final_mark_slash_impl(fields)
}

fn prefix_before_final_mark_slash_impl(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .len()
}
