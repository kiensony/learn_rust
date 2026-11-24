# Review notes

`observe_universal_slice_sum` returns an owned Vec<u32>. It reads the borrowed input and records universal slice sum in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The observed scalar is copied while the current chunk's borrow is live. The collected vector owns its values independently of every borrowed chunk.
