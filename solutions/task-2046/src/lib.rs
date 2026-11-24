// rust-track: handwritten task-2046
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2046;

pub fn polled_lengths_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
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
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
