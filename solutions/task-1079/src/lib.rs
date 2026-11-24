// rust-track: handwritten task-1079
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1079;

pub trait Packet {
    type Payload: AsRef<str>;
    fn payload(&self) -> &Self::Payload;
}
impl Packet for String {
    type Payload = String;
    fn payload(&self) -> &String {
        self
    }
}
impl<'a> Packet for &'a str {
    type Payload = &'a str;
    fn payload(&self) -> &&'a str {
        self
    }
}

pub fn longest_field_with_last_tie_pipe(packet: &impl Packet) -> usize {
    longest_field_with_last_tie_pipe_impl(packet)
}

fn longest_field_with_last_tie_pipe_impl(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
