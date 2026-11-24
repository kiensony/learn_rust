# Review notes

`observe_atomic_running_total` returns an owned Vec<u32>. It reads the borrowed input and records atomic running total in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Each atomic operation returns the previous value of a single cell. No other memory is published by that value, so the trace needs atomicity and imposes no cross-location ordering requirement.
