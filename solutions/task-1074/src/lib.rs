// rust-track: handwritten task-1074
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1074;

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

pub fn closing_marks_removed_pipe(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    })
    .len()
}
