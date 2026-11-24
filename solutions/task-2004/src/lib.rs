// rust-track: handwritten task-2004
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2004;

pub fn polled_lengths_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::future::Future;
        use std::task::{Context, Poll, Waker};
        let mut cx = Context::from_waker(Waker::noop());
        input
            .iter()
            .map(|&x| {
                let f = std::future::ready(u32::from(x));
                let mut pin = std::pin::pin!(f);
                match pin.as_mut().poll(&mut cx) {
                    Poll::Ready(v) => v,
                    Poll::Pending => unreachable!(),
                }
            })
            .collect()
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
