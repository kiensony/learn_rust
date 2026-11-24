# Review notes

`observe_locked_batch_peaks` returns an owned Vec<u32>. It reads the borrowed input and records locked batch peaks in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
