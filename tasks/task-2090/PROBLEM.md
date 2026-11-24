# task-2090 — Drain a bounded inbox on shutdown

A service stops accepting requests during shutdown. A successful send acknowledges that the service will preserve that record. Several clients report missing records after an orderly shutdown.

Run in this directory with Rust 1.88.0. Fetch the locked dependencies once with `cargo +1.88.0 fetch --locked`.

1. Reproduce the loss with `cargo +1.88.0 test --locked --test public shutdown_drains_the_whole_queue`. Record the expected and actual lists.
2. Identify the point where the receiver stops accepting sends and the separate point where it stops returning records.
3. Repair `pub async fn shutdown(mut inbox: Receiver<u32>) -> Vec<u32>` in `src/lib.rs`. Preserve the API and the bounded Tokio channel.
4. Run `cargo +1.88.0 test --locked`. Write a regression case in a new integration test using a queue of capacity two and an outstanding reservation.

After shutdown begins the receiver must close to new reservations. Return all accepted records in receive order exactly once, including repeated values and records subsequently delivered through already issued permits. Complete after every issued permit has been used or released, even if idle senders remain. The tests use a paused clock to bound waits. [Tokio receiver documentation](https://docs.rs/tokio/1.49.0/tokio/sync/mpsc/struct.Receiver.html)
